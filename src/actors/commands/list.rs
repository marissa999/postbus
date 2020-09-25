use futures::Future;
use stage::actor_msg;
use stage::actors::{Actor, ActorCtx, ActorResult, Message, Response};
use telegram_bot_async::{InlineKeyboardButton, MessageChat};
use uuid::Uuid;

use crate::actors::{CallbackRouter, EditMessage, Scheduler, TelegramSender};
use crate::actors::callback_router::{Callback, RegisterCallback};
use crate::actors::command_handler::{CreateCommand, DestroyCommand};
use crate::actors::telegram_sender::{MessageResponse, SendMessage};
use crate::actors::scheduler::GetEvents;
use crate::actors::database::EventsResponse;
use crate::database::models::Event;

pub struct List {
  message: Option<telegram_bot_async::Message>,

  telegram_sender: Actor,
  callback_router: Actor,
  scheduler: Actor,

  events: Vec<Event>,
  button_id: Uuid,
}

impl List {
  pub fn new(telegram_sender: Actor, callback_router: Actor, scheduler: Actor) -> Self {
    List {
      message: None,

      telegram_sender,
      callback_router,
      scheduler,

      events: Vec::new(),
      button_id: Uuid::new_v4(),
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(msg) = msg.try_cast::<CreateCommand>() {
      let events = self.scheduler.ask(GetEvents { chat: msg.chat.id().0 }, Response::Wait).await.expect("scheduler exists").unwrap();
      let events = events.try_cast::<EventsResponse>().unwrap();
      self.events = events.events.clone();


      self.callback_router.send(RegisterCallback::create(self.button_id, ctx.this.clone(), false)).await.expect("CallbackRouter is never dead");
      let msg = self.telegram_sender.ask(SendMessage {
        chat: msg.chat.clone(),
        text: "Menu message".to_string(),
        keyboard: Some(vec![
          vec![InlineKeyboardButton::callback("test", &self.button_id.to_string())]
        ].into()),
      }, Response::Wait).await.expect("").unwrap();

      if let Some(msg) = msg.try_cast::<MessageResponse>() {
        self.message = Some(msg.message.to_owned());
      }
    } else if let Some(msg) = msg.try_cast::<Callback>() {
      match msg.id {
        x if x == self.button_id => {
          if let Some(msg) = &self.message {
            let msg = self.telegram_sender.ask(EditMessage {
              chat: msg.chat.clone(),
              message: msg.id,
              text: "Menu message".to_string(),
              keyboard: Some(vec![
                vec![InlineKeyboardButton::callback("ok", &self.button_id.to_string())]
              ].into()),
            }, Response::Wait).await.expect("could not edit").unwrap();

            if let Some(msg) = msg.try_cast::<MessageResponse>() {
              self.message = Some(msg.message.to_owned());
            }
          }
          println!("Pressed button");
        }
        _ => {}
      }
    } else if let Some(msg) = msg.try_cast::<DestroyCommand>() {
      println!("Destroy {}", self.button_id);
      if let Some(msg) = &self.message {
        self.telegram_sender.send(EditMessage {
          chat: msg.chat.clone(),
          message: msg.id,
          keyboard: None,
          text: "Menu message".to_string(),
        }).await.expect("TelegramSender should always exist");
      }
      ctx.sys.stop_actor(ctx.this.get_ref().clone());
    }

    return Ok(self);
  }
}
