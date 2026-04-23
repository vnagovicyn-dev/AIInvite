ALTER TABLE guests
ADD COLUMN invite_token TEXT;

UPDATE guests
SET invite_token = substr(replace(gen_random_uuid()::text, '-', ''), 1, 24)
WHERE invite_token IS NULL;

ALTER TABLE guests
ALTER COLUMN invite_token SET NOT NULL;

ALTER TABLE guests
ADD CONSTRAINT guests_invite_token_key UNIQUE (invite_token);
