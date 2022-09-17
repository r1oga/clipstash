create table if not exists clips
(
    clip_id   text primary key not null ,
    shortcode text unique not null,
    content   text not null,
    title     text,
    posted    datetime not null,
    expires   datetime,
    password  TEXT,
    hits      bigint not null
);