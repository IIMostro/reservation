-- Add up migration script here
CREATE TYPE rsvp.reservation_status as ENUM('unknown','pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type as ENUM('unknown', 'create', 'update', 'delete');
CREATE TABLE rsvp.reservations
(
    id          uuid                    not null default gen_random_uuid(),
    user_id     varchar(64)             not null,
    status      rsvp.reservation_status not null default 'pending',
    resource_id varchar(64)             not null,
    -- 时间的区间
    timespan    tstzrange               not null,
    note        text,

    -- 设置id为主键
    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    -- postgres下的一个索引，如果一个resource_id等于并且在另一个预定区间的话则会触发索引，无法插入
    -- CONSTRAINT表示插入的字段与已有的字段检查，判断是否可以插入
    -- EXCLUDE是行的表现，现有的行与已有的行的关系
    -- gist是索引类型，gist索引可以有很多别的操作符号,多维的查询条件
    -- 例如 @> && 等
    CONSTRAINT  reservations_conflict EXCLUDE using gist (resource_id WITH =, timespan WITH &&)
);

CREATE INDEX reservations_resource_id_idx on rsvp.reservations (resource_id);
CREATE INDEX reservations_user_id_idx on rsvp.reservations (user_id);