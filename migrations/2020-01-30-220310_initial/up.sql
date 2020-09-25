CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE message_list
(
    id   uuid PRIMARY KEY   NOT NULL DEFAULT uuid_generate_v4(),

    name varchar(64) UNIQUE NOT NULL
);

CREATE TABLE message
(
    id       uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),

    message  varchar(512)     NOT NULL,
    approved bool             NOT NULL,

    list     uuid references message_list (id)
);

CREATE TABLE chat
(
    id                       bigint PRIMARY KEY NOT NULL,

    default_annoying         bool               NOT NULL DEFAULT false,
    default_annoying_minutes int
);

CREATE TABLE event
(
    id               uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),

    hour             int              NOT NULL CHECK ( hour >= 0 AND hour <= 23 ),
    minute           int              NOT NULL CHECK ( hour >= 0 AND hour <= 59 ),

    chat             bigint references chat (id) NOT NULL,

    message_list     uuid references message_list (id),
    message          varchar(512),

    annoying         bool             NOT NULL DEFAULT false,
    annoying_minutes int,

    CHECK ( message_list IS NOT NULL OR message IS NOT NULL )
);
--
-- INSERT INTO chat
--     VALUES (111)
-- ON CONFLICT DO NOTHING
--     RETURNING id;
--
-- INSERT INTO event
--     VALUES (uuid_generate_v4(),
--             12,
--             50,
--             111,
--             null,
--             null,
--             null,
--             null);
