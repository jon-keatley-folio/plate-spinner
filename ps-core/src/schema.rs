
pub const PS_V1_SCHEMA:String = "
CREATE SCHEMA IF NOT EXISTS psv1;

CREATE SEQUENCE IF NOT EXISTS psv1.plate_sequence START WITH 1 INCREMENT BY 1;

CREATE TABLE IF NOT EXISTS psv1.plates(
    id INTEGER PRIMARY KEY DEFAULT nextval('psv1.plate_sequence'),
    title VARCHAR,
    description VARCHAR,
    frequency INTERVAL,
    next DATE,
    started DATE,
    saved UINT32,
    spinning BOOL, 
)
";

