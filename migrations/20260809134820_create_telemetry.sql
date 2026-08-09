CREATE TABLE telemetry (
  id UUID PRIMARY KEY,
  device_id UUID NOT NULL,
  key VARCHAR(100) NOT NULL,
  value DOUBLE PRECISION NOT NULL,
  unit VARCHAR(50) NOT NULL,
  recorded_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT fk_telemetry_device
    FOREIGN KEY (device_id) REFERENCES devices(id)
);

CREATE INDEX idx_telemetry_device_recorded ON telemetry (device_id, recorded_at);
