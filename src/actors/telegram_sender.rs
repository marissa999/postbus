use futures::Future;
use stage::actor_msg;
use stage::actors::{ActorCtx, ActorResult, Message};
use stage::sys_msgs::Request;
use telegram_bot_async::{CallbackQuery, CanAnswerCallbackQuery, DefaultApi, InlineKeyboardMarkup, MessageChat, MessageId};
use tracing::warn;

/// -> telegram_bot_async::Message
pub struct SendMessage {
  pub chat: MessageChat,
  pub text: String,
  pub keyboard: Option<InlineKeyboardMarkup>,
}

/// -> telegram_bot_async::Message
pub struct EditMessage {
  pub chat: MessageChat,
  pub message: MessageId,
  pub text: String,
  pub keyboard: Option<InlineKeyboardMarkup>,
}

pub struct AnswerCallback {
  pub callback: CallbackQuery,
}

pub struct MessageResponse {
  pub message: telegram_bot_async::Message,
}

actor_msg!(SendMessage, EditMessage, AnswerCallback, MessageResponse);

pub struct TelegramSender {
  api: DefaultApi,
}

impl TelegramSender {
  pub fn new(api: DefaultApi) -> Self {
    TelegramSender {
      api
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(req) = msg.try_cast::<Request>() {
      if let Some(msg) = req.msg().try_cast::<SendMessage>() {
        let mut to_send = telegram_bot_async::SendMessage::new(&msg.chat, &msg.text);
        if let Some(kb) = &msg.keyboard { to_send = to_send.reply_markup(kb.to_owned()); }

        let message: telegram_bot_async::Message = self.api.send(to_send).await.expect("should be able to send to telegram");
        req.respond(MessageResponse { message });
      } else if let Some(msg) = req.msg().try_cast::<EditMessage>() {
        let mut to_send = telegram_bot_async::EditMessageText::new(&msg.chat, msg.message, &msg.text);
        if let Some(kb) = &msg.keyboard { to_send = to_send.reply_markup(kb.to_owned()); }

        let message: telegram_bot_async::Message = self.api.send(to_send).await.expect("should be able to send to telegram");
        req.respond(MessageResponse { message });
      } else {
        warn!("unexpected request received");
      }
    } else if let Some(msg) = msg.try_cast::<AnswerCallback>() {
      self.api.send(msg.callback.acknowledge()).await.expect("should be able to send to telegram");
    } else {
      warn!("unexpected message received");
    }

    return Ok(self);
  }
}
