CREATE TABLE guests (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    full_name TEXT NOT NULL,
    phone TEXT NULL,
    email TEXT NULL,
    group_name TEXT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    plus_one_allowed BOOLEAN NOT NULL DEFAULT FALSE,
    is_child BOOLEAN NOT NULL DEFAULT FALSE,
    vip BOOLEAN NOT NULL DEFAULT FALSE,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guests_event_id_created_at ON guests (event_id, created_at DESC);
CREATE INDEX idx_guests_event_id_vip ON guests (event_id, vip);
CREATE INDEX idx_guests_event_id_group_name ON guests (event_id, group_name);
