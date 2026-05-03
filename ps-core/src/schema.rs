
pub const PS_V1_SCHEMA:&str = "
CREATE SCHEMA IF NOT EXISTS psv1;

CREATE SEQUENCE IF NOT EXISTS psv1.plate_sequence START WITH 1 INCREMENT BY 1;

CREATE TABLE IF NOT EXISTS psv1.plates(
    id INTEGER PRIMARY KEY DEFAULT nextval('psv1.plate_sequence'),
    title VARCHAR,
    description VARCHAR,
    frequency INTERVAL,
    next DATE,
    started DATE,
    saved UINT32 default 0,
    spinning BOOL default true, 
);

PREPARE add_plate AS INSERT INTO psv1.plates (title, description, frequency, next, started) VALUES ($1, $2, $3, $4, $5);

PREPARE update_plate AS UPDATE psv1.plates 
SET title = $1, description = $2, frequency = $3
WHERE psv1.plates.id = $4;

PREPARE pause_plate AS UPDATE psv1.plates SET spinning=false
WHERE psv1.plates.id = $1;

PREPARE start_spinning_plate AS UPDATE psv1.plates SET spinning=true
WHERE psv1.plates.id = $1;

PREPARE spin_plate AS UPDATE psv1.plates SET next = today() + frequency, saved = saved + 1
WHERE psv1.plates.id = $1;

PREPARE top_topples as SELECT * FROM psv1.plates WHERE psv1.plates.spinning = true
ORDER BY next limit $1;

PREPARE paused_plates as SELECT * FROM psv1.plates WHERE psv1.plates.spinning = false
ORDER BY next limit $1;

PREPARE list_plates as SELECT * FROM psv1.plates
ORDER BY next limit $1 OFFSET $2;
";

/*
execute add_plate('test', 'a test plate', '2 days', '2026-5-5', '2026-5-3'); 
*/

