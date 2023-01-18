-- Add up migration script here
create table rsvp.reservation_changes
(
    id             SERIAL                       NOT NULL,
    reservation_id uuid                         not null,
    -- 数据约束在这几个单词之间
    op             rsvp.reservation_update_type not null
);

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