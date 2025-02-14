CREATE TABLE machine_logs (
    id SERIAL PRIMARY KEY,
    machine_id VARCHAR(100) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    raw_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE metrics (
    id SERIAL PRIMARY KEY,
    log_id INTEGER REFERENCES machine_logs(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    CONSTRAINT unique_metric_per_log UNIQUE (log_id, metric_name)
);

-- Indexes for performance 
CREATE INDEX idx_machine_logs_machine_id ON machine_logs(machine_id);
CREATE INDEX idx_machine_logs_timestamp ON machine_logs(timestamp);
CREATE INDEX idx_metrics_name_value ON metrics(metric_name, metric_value);


