use std::collections::HashMap;

use stage::actor_msg;
use stage::actors::{Actor, ActorCtx, ActorResult, Message};
use telegram_bot_async::CallbackQuery;
use uuid::Uuid;

use crate::actors::telegram_sender::AnswerCallback;
use crate::actors::TelegramSender;

pub struct RegisterCallback {
  pub id: Uuid,
  pub source: Actor,
  pub normal_messages: bool,
}

impl RegisterCallback {
  pub fn create(id: Uuid, source: Actor, normal_messages: bool) -> Self {
    RegisterCallback {
      id,
      source,
      normal_messages,
    }
  }
}

pub struct RouteCallback {
  pub query: CallbackQuery,
}

pub struct Callback {
  pub id: Uuid,
}

actor_msg!(RegisterCallback, RouteCallback, Callback);

pub struct CallbackRouter {
  telegram_sender: Actor,

  map: HashMap<Uuid, Actor>,
}

impl CallbackRouter {
  pub fn new(telegram_sender: Actor) -> Self {
    CallbackRouter {
      telegram_sender,
      map: HashMap::new(),
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(msg) = msg.try_cast::<RegisterCallback>() {
      self.map.insert(msg.id, msg.source.clone());
    } else if let Some(msg) = msg.try_cast::<RouteCallback>() {
      let id = match Uuid::parse_str(&msg.query.data) {
        Ok(id) => id,
        Err(err) => {
          eprintln!("Failed parsing callback Uuid from Telegram {:?}: {:?}", msg.query.data, err);
          return Ok(self);
        }
      };
      if let Some(chan) = self.map.get(&id) {
        if chan.send(Callback { id }).await.is_ok() {
          self.telegram_sender.send(AnswerCallback {
            callback: msg.query.clone(),
          }).await.expect("TelegramSender should always exist");
        } else {
          self.map.remove(&id);
        }
      }
    }

    return Ok(self);
  }
}
