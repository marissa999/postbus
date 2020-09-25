use std::time::Duration;

use chrono::{NaiveTime, Timelike, Utc};
use stage::actor_msg;
use stage::actors::{Actor, ActorCtx, ActorResult, Message, Response};
use stage::sys_msgs::ActorStart;
use tracing::warn;

use crate::actors::{GetAllScheduledEvents};
use crate::database::models::Event;
use crate::actors::database::GetScheduledEvents;

/// Initial fetch from Database
pub struct FetchScheduledEvents {}

pub struct ScheduleEvent {
  event: Event,
}

pub struct CreateEvent {}

pub struct ScheduleTmpEvent {}

pub struct RemoveTmpEvents {}

pub struct RemoveEvent {}

/// -> Vec<Event>
pub struct GetEvents {
  pub chat: i64
}

pub struct HandleEvent {
  pub event: Event,
}

actor_msg!(FetchScheduledEvents, ScheduleEvent, CreateEvent, ScheduleTmpEvent, RemoveTmpEvents, RemoveEvent, GetEvents, HandleEvent);

pub struct Scheduler {
  database: Actor,
  telegram_sender: Actor,
  callback_router: Actor,
}

impl Scheduler {
  pub fn new(database: Actor, telegram_sender: Actor, callback_router: Actor) -> Self {
    Scheduler {
      database,
      telegram_sender,
      callback_router,
    }
  }

  pub async fn handle(mut self, ctx: ActorCtx, msg: Message) -> ActorResult<Self> {
    if let Some(msg) = msg.try_cast::<ActorStart>() {
      ctx.this.send(FetchScheduledEvents {}).await.unwrap();
    } else if let Some(msg) = msg.try_cast::<FetchScheduledEvents>() {
      let events = self.database.ask(GetAllScheduledEvents {}, Response::Wait).await.expect("Should be able to get events");

      for event in events {
//                ctx.notify_immediately(ScheduleEvent { event });
      }
    } else if let Some(msg) = msg.try_cast::<ScheduleEvent>() {
      let time = NaiveTime::from_hms(msg.event.hour as u32, msg.event.minute as u32, 0);
      let dur = duration_until(time);
      ctx.this.send_after(HandleEvent { event: msg.event.clone() }, dur).expect("should be able to queue a message for itself");
    } else if let Some(msg) = msg.try_cast::<CreateEvent>() {
      unimplemented!();
    } else if let Some(msg) = msg.try_cast::<ScheduleTmpEvent>() {
      unimplemented!();
    } else if let Some(msg) = msg.try_cast::<RemoveTmpEvents>() {
      unimplemented!();
    } else if let Some(msg) = msg.try_cast::<RemoveEvent>() {
      unimplemented!();
    } else if let Some(msg) = msg.try_cast::<GetEvents>() {
      self.database.ask(GetScheduledEvents { chat: msg.chat }, Response::Wait).await.expect("should be able to get events for chat");
      unimplemented!();
    } else if let Some(msg) = msg.try_cast::<HandleEvent>() {
      unimplemented!();
    } else {
      warn!("received unknown event");
    }

    return Ok(self);
  }
}

fn duration_until(time: NaiveTime) -> Duration {
  let now = Utc::now();

  let secs_from_midnight = time.num_seconds_from_midnight();
  let now_secs_from_midnight = now.time().num_seconds_from_midnight();
  let secs_until_event = if now_secs_from_midnight > secs_from_midnight {
    secs_from_midnight + (86400 - now_secs_from_midnight)
  } else {
    secs_from_midnight - now_secs_from_midnight
  };

  Duration::from_secs(secs_until_event as u64)
}
