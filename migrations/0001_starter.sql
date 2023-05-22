create table sections (
    id serial primary key,
    title varchar(80) not null,
    description varchar(255) not null,
    parent_section_id smallint references sections(id)
);

create table topics (
    id serial primary key,
    title text not null,
    parent_section_id integer not null references sections(id),
    character_id integer not null,
    character_name varchar(70) not null,
    character_avatar varchar(10) not null,
    content text not null,
    words_count integer not null,
    created_at timestamp not null default now(),
    updated_at timestamp
);

create table posts (
    id serial primary key,
    topic_id integer not null references topics(id),
    character_id integer not null,
    character_name varchar(70) not null,
    character_avatar varchar(10) not null,
    content text not null,
    words_count integer not null,
    created_at timestamp not null default now(),
    updated_at timestamp
);
