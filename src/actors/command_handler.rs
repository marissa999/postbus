use std::collections::HashMap;

use stage::actor_msg;
use stage::actors::{ActorCtx, ActorResult, Message, Actor};
use telegram_bot_async::MessageChat;

use crate::actors::{CallbackRouter, commands::List, Scheduler, TelegramSender};
use crate::create_actor;
use futures::executor::block_on;

pub struct TelegramCommand {
  pub chat: MessageChat,
  pub command: String,
}

pub struct CreateCommand {
  pub chat: MessageChat,
}

pub struct DestroyCommand {}

actor_msg!(TelegramCommand, CreateCommand, DestroyCommand);

pub struct CommandHandler {
  telegram_sender: Actor,
  callback_router: Actor,
  scheduler: Actor,

  map: HashMap<MessageChat, HashMap<Command, Actor>>,
}

impl CommandHandler {
  pub fn new(telegram_sender: Actor, callback_router: Actor, scheduler: Actor) -> Self {
    CommandHandler {
      telegram_sender,
      callback_router,
      scheduler,

      map: HashMap::new(),
    }
  }
  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(msg) = msg.try_cast::<TelegramCommand>() {
      let chat_commands = self.map.entry(msg.chat.clone()).or_insert_with(HashMap::new);
      let command = Command::parse(&msg.command);

      match command {
        Command::List => {
          let addr = create_actor(&ctx.sys,List::new(self.telegram_sender.clone(), self.callback_router.clone(), self.scheduler.clone()), List::handle);
          addr.send(CreateCommand { chat: msg.chat.clone() }).await.expect("should be able to send a message to an actor just created");
          Some(addr)
        }
        Command::Reminder => {
          unimplemented!()
        }
        Command::_NoCommand_ => {
          None
        }
      }.and_then(|ch| {
        chat_commands.insert(command, ch)
      }).map(|ch| {
        block_on(ch.send(DestroyCommand {})).is_ok()
      });
    }

    return Ok(self);
  }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Command {
  List,
  Reminder,

  _NoCommand_,
}

impl Command {
  fn parse(command: &str) -> Self {
    match () {
      _ if command == "/list" => Command::List,
      _ if command == "/reminder" => Command::Reminder,
      _ => Command::_NoCommand_
    }
  }
}
