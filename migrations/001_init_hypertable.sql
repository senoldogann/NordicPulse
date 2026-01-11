-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create IoT Data Table
CREATE TABLE IF NOT EXISTS iot_data (
    time        TIMESTAMPTZ NOT NULL,
    device_id   TEXT NOT NULL,
    device_type TEXT NOT NULL,
    location    TEXT NOT NULL,
    value       DOUBLE PRECISION NOT NULL,
    unit        TEXT NOT NULL
);

-- Convert to Hypertable (partition by time)
SELECT create_hypertable('iot_data', 'time', if_not_exists => TRUE);

-- Add Index for frequent queries (device_id + time)
CREATE INDEX IF NOT EXISTS idx_iot_data_device_time ON iot_data (device_id, time DESC);
