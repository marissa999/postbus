
CREATE TABLE new_message_list
(
    id   uuid PRIMARY KEY NOT NULL,

    name varchar(64) UNIQUE NOT NULL
);
INSERT INTO new_message_list SELECT * FROM message_list;
DROP TABLE IF EXISTS message_list CASCADE;
ALTER TABLE new_message_list RENAME TO message_list;

CREATE TABLE new_message
(
    id       uuid PRIMARY KEY NOT NULL,

    message  varchar(512) NOT NULL,
    approved bool NOT NULL,

    list     uuid references message_list (id)
);
INSERT INTO new_message SELECT * FROM message;
DROP TABLE IF EXISTS message CASCADE;
ALTER TABLE new_message RENAME TO message;

CREATE TABLE new_event
(
    id           uuid PRIMARY KEY NOT NULL,

    hour         int NOT NULL CHECK ( hour >= 0 AND hour <= 23 ),
    minute       int NOT NULL CHECK ( hour >= 0 AND hour <= 59 ),

    chat         bigint NOT NULL,

    message_list uuid references message_list (id) ,
    message      uuid references message (id),

    CHECK ( message_list IS NOT NULL OR message IS NOT NULL )
);
INSERT INTO new_event SELECT * FROM event;
DROP TABLE IF EXISTS event CASCADE;
ALTER TABLE new_event RENAME TO event;
