table! {
  chat (id) {
    id -> Int8,
    default_annoying -> Bool,
    default_annoying_minutes -> Nullable<Int4>,
  }
}

table! {
  event (id) {
    id -> Uuid,
    hour -> Int4,
    minute -> Int4,
    chat -> Int8,
    message_list -> Nullable<Uuid>,
    message -> Nullable<Varchar>,
    annoying -> Bool,
    annoying_minutes -> Nullable<Int4>,
  }
}

table! {
  message (id) {
    id -> Uuid,
    #[sql_name = "message"]
    msg -> Varchar,
    approved -> Bool,
    list -> Nullable<Uuid>,
  }
}

table! {
  message_list (id) {
    id -> Uuid,
    name -> Varchar,
  }
}

joinable!(event -> chat (chat));
joinable!(event -> message_list (message_list));
joinable!(message -> message_list (list));

allow_tables_to_appear_in_same_query!(
  chat,
  event,
  message,
  message_list,
);
