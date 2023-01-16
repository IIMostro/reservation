### 数据库

数据库的设计schema

```sql

-- postgres 的schema隔离作用
CREATE SCHEMA rsvp;

CREATE TYPE rsvp.reservation_status as ENUM('unknown','pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type as ENUM('unknown', 'create', 'update', 'delete');

CREATE TABLE rsvp.reservations
(
    id          uuid                    not null default uuid_generate_v4(),
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

-- reservation change queue
-- 当主表reservation修改时会把缓存数据放到这个表里面
create table rsvp.reservation_changes
(
    id             SERIAL                       NOT NULL,
    reservation_id uuid                         not null,
    -- 数据约束在这几个单词之间
    op             rsvp.reservation_update_type not null
)

-- 如果uid为空的话则查询所有的资源列表
-- 如果rid为空的话也这么查询
CREATE
OR REPLACE FUNCTION rsvp.query(uid, text,rid, during: tstzrange) RETURN TABLE rsvp.reservations as $$ $$ LANGUAGE plpgsql;
CREATE
OR REPLACE FUNCTION rsvp.reservations_trigger() returns trigger as 
    $$
BEGIN 
        IF TG_OP = 'INSERT' then
                    INSERT INTO rsvp.reservation_changes(reservation_id, op) 
                    VALUES (NEW.id, 'create');
        ELSIF TG_OP = 'UPDATE' then
            IF OLD.status <> NEW.status then
                INSERT INTO rsvp.reservation_changes(reservation_id, op) 
                VALUES (NEW.id, 'update');
            END IF;
        ELSIF TG_OP = 'DELETE' then
                    INSERT INTO rsvp.reservation_changes(reservation_id, op) 
                    VALUES (NEW.id, 'delete');
        END IF;
                -- postgres的通知功能
                NOTIFY
        reservation_update;
        return NULL;
END;
    $$
LANGUAGE plpgsql;
    
    
CREATE TRIGGER reservations_trigger
    after insert or
update or
delete
on rsvp.reservations for each row
    execute procedure rsvp.reservations_trigger();  
```