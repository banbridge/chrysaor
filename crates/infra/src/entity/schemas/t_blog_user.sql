create table t_users
(
    id         bigserial,
    username   varchar(128)                                       not null,
    user_id    varchar(128)                                       not null,
    nickname   varchar(128),
    created_at timestamp with time zone                           not null,
    updated_at timestamp with time zone default CURRENT_TIMESTAMP not null
);

comment on column t_users.id is 'ID';
comment on column t_users.username is '用户名称';
comment on column t_users.user_id is '用户ID';
comment on column t_users.nickname is '用户昵称';
comment on column t_users.created_at is '创建时间';
comment on column t_users.updated_at is '更新时间';

create unique index idx_user_id
    on t_users (user_id);

create unique index t_users_pkey
    on t_users (id);

