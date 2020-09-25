use std::ops::Add;

use futures::Future;
use stage::actor_msg;
use stage::actors::{ActorCtx, ActorResult, Message};
use stage::sys_msgs::Request;
use tracing::warn;

use crate::database::models::Event;

/// -> Vec<Event>
pub struct GetScheduledEvents {
  pub chat: i64
}

/// -> Vec<Event>
pub struct GetAllScheduledEvents {}

/// -> Event
pub struct AddScheduledEvent {}

pub struct EventsResponse {
  pub events: Vec<Event>,
}

actor_msg!(GetScheduledEvents, GetAllScheduledEvents, AddScheduledEvent, EventsResponse);

pub struct Database {
  db: crate::database::Database,
}

impl Database {
  pub fn new(db: crate::database::Database) -> Self {
    Database {
      db
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(req) = msg.try_cast::<Request>() {
      if let Some(msg) = req.msg().try_cast::<GetScheduledEvents>() {
        req.respond(EventsResponse { events: self.db.get_scheduled_events(msg.chat) });
      } else if let Some(msg) = req.msg().try_cast::<GetAllScheduledEvents>() {
        req.respond(EventsResponse { events: self.db.get_all_scheduled_events() });
      } else {
        warn!("unexpected request received");
      }
    } else if let Some(msg) = msg.try_cast::<GetAllScheduledEvents>() {
      warn!("memed");
    } else if let Some(msg) = msg.try_cast::<AddScheduledEvent>() {
      // let mut conn = self.pool.acquire().await.unwrap();
      //
      // let event = sqlx::query(
      //   r#"
      //              INSERT INTO event ( id, hour, minute, chat, message_list, message )
      //              VALUES ( $1, $2, $3, $4, $5, $6 )
      //              RETURNING id, hour, minute, chat, message_list, message
      //          "#,
      // ).fetch_one(&mut conn)
      //   .await
      //   .unwrap();
      //
      // event
      unimplemented!();
    } else {
      warn!("unexpected message received");
    }

    return Ok(self);
  }
}
