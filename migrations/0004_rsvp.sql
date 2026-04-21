CREATE TABLE rsvp_forms (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL UNIQUE REFERENCES events(id) ON DELETE CASCADE,
    title TEXT NOT NULL DEFAULT 'Подтверждение участия',
    description TEXT NULL,
    deadline_at TIMESTAMPTZ NULL,
    settings JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE rsvp_questions (
    id UUID PRIMARY KEY,
    form_id UUID NOT NULL REFERENCES rsvp_forms(id) ON DELETE CASCADE,
    position INT NOT NULL,
    code TEXT NOT NULL,
    label TEXT NOT NULL,
    question_type TEXT NOT NULL,
    required BOOLEAN NOT NULL DEFAULT FALSE,
    options JSONB NOT NULL DEFAULT '[]'::jsonb,
    UNIQUE(form_id, code)
);

CREATE INDEX idx_rsvp_questions_form_position
    ON rsvp_questions(form_id, position);

CREATE TABLE rsvp_responses (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    guest_id UUID NULL REFERENCES guests(id) ON DELETE SET NULL,
    public_token TEXT NULL,
    status TEXT NOT NULL,
    plus_one_count INT NOT NULL DEFAULT 0,
    answers JSONB NOT NULL DEFAULT '{}'::jsonb,
    submitted_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_rsvp_responses_event_created_at
    ON rsvp_responses(event_id, created_at DESC);

CREATE INDEX idx_rsvp_responses_event_status
    ON rsvp_responses(event_id, status);

CREATE INDEX idx_rsvp_responses_event_guest_id
    ON rsvp_responses(event_id, guest_id);
