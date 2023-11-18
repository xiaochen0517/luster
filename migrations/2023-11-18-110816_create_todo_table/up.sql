-- Your SQL goes here
create table todo_table
(
    id          integer primary key autoincrement,
    title       varchar(100) not null,
    description varchar(500) null,
    due_date    date         null,
    completed   boolean      not null default false,
    create_time timestamp    not null default current_timestamp
);