-- Add up migration script here
CREATE OR REPLACE FUNCTION rsvp.query(uid text,rid text, during tstzrange) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
    BEGIN
        IF uis IS NULL AND rid IS NULL THEN
            RETURN QUERY SELECT * FROM rsvp.reservations WHERE during && timespan;
        ELSEIF uid IS NULL THEN
            RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND during @> timespan;
        ELSEIF rid IS NULL THEN
            RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND during @> timespan;
        ELSE
            RETURN QUERY SELECT * FROM rsvp.reservations WHERE uid = uid AND rid = rid AND during && during;
        END IF;
    END;
$$ LANGUAGE plpgsql;