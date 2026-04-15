CREATE TABLE IF NOT EXISTS lead_requests (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    event_date TEXT NOT NULL,
    event_type TEXT NOT NULL,
    comment TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
