CREATE TABLE devices (
    id CHAR(36) PRIMARY KEY
);

CREATE TABLE pings (
    epoch_time BIGINT PRIMARY KEY,
    device_id CHAR(36) NOT NULL REFERENCES devices (id)
);
