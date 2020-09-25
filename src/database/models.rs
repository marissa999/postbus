use super::schema::event;

#[derive(Clone, Debug, Queryable)]
pub struct Event {
  pub id: uuid::Uuid,

  pub hour: i32,
  pub minute: i32,

  pub chat: i64,

  pub message_list: Option<uuid::Uuid>,
  pub message: Option<String>,

  pub annoying: bool,
  pub annoying_minutes: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "event"]
pub struct NewEvent {
  pub hour: i32,
  pub minute: i32,

  pub chat: i64,

  pub message: String,
}

#[derive(Clone, Debug, Queryable)]
pub struct Message {
  pub id: uuid::Uuid,

  pub message: String,
  pub approved: bool,

  pub list: Option<uuid::Uuid>,
}

#[derive(Clone, Debug, Queryable)]
pub struct MessageList {
  pub id: uuid::Uuid,

  pub name: String,
}


