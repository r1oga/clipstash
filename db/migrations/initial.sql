create table if not exists clips
(
    clip_id   text primary key not null ,
    shortcode text not null,
    content   text not null,
    title     text,
    posted    datetime not null,
    expires   datime not null,
    password  TEXT,
    hits      bigint not null
);