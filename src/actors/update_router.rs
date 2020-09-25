use stage::actor_msg;
use stage::actors::{Actor, ActorCtx, ActorResult, Message};
use telegram_bot_async::{MessageEntityKind, MessageKind, UpdateKind};
use tracing::info;

use crate::actors::callback_router::RouteCallback;
use crate::actors::command_handler::{TelegramCommand};
use crate::actors::telegram_sender::TelegramSender;

pub struct Update {
  pub update: telegram_bot_async::Update,
}

actor_msg!(Update);

pub struct UpdateRouter {
  telegram_sender: Actor,
  callback_router: Actor,
  command_handler: Actor,
}

impl UpdateRouter {
  pub fn new(telegram_sender: Actor, callback_router: Actor, command_handler: Actor) -> Self {
    UpdateRouter {
      telegram_sender,
      callback_router,
      command_handler,
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(msg) = msg.try_cast::<Update>() {
      info!(id = msg.update.id ,"router");
      match &msg.update.kind {
        UpdateKind::Message(msg) => {
          info!("message {:?}", &msg);
          if let MessageKind::Text { data, entities } = &msg.kind {
            let command = entities
              .iter()
              .find(|e| if let MessageEntityKind::BotCommand = e.kind { e.offset == 0 } else { false });
            if command.is_some() {
              self.command_handler.send(TelegramCommand {
                command: data.clone(),
                chat: msg.chat.clone(),
              }).await.expect("The CommandHandler actor should always be alive");
            }
          }
        }
        UpdateKind::CallbackQuery(cb) => {
          info!("callback {:?}", &cb);
          self.callback_router.send(RouteCallback {
            query: cb.clone()
          }).await.expect("The CallbackRouter actor should always be alive");
        }
        _ => {}
      }
    }

    return Ok(self);
  }
}
