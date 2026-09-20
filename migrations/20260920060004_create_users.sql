-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    profile_pic TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT users_email_unique UNIQUE (email),
    CONSTRAINT users_email_normalized
        CHECK (email = lower(btrim(email))),
    CONSTRAINT users_email_length
        CHECK (char_length(email) BETWEEN 3 AND 254)
);