-- Add migration script here
CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "sanitized_email" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "user_events" (
    "id" UUID NOT NULL REFERENCES users(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);

CREATE TYPE "event_state" AS ENUM ('unapproved', 'live', 'rejected', 'deleted');

CREATE TABLE IF NOT EXISTS "events" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "start_date" DATE NOT NULL,
    "organizer_id" UUID REFERENCES users(id),
    "state" event_state NOT NULL DEFAULT 'unapproved',
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "event_events" (
    "id" UUID NOT NULL REFERENCES events(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);

CREATE TABLE IF NOT EXISTS "days" (
    "id" UUID PRIMARY KEY,
    "event_id" UUID NOT NULL REFERENCES events(id),
    "day_number" SMALLINT NOT NULL,
    "start_time" TIME,
    "end_time" TIME,
    "created_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (event_id, day_number)
);

CREATE TABLE IF NOT EXISTS "day_events" (
    "id" UUID NOT NULL REFERENCES days(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);

CREATE TABLE IF NOT EXISTS "stages" (
    "id" UUID PRIMARY KEY,
    "event_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id)
);

CREATE TABLE IF NOT EXISTS "stage_events" (
    "id" UUID NOT NULL REFERENCES stages(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);

CREATE TABLE IF NOT EXISTS "acts" (
    "id" UUID PRIMARY KEY,
    "event_id" UUID NOT NULL,
    "stage_id" UUID,
    "name" TEXT NOT NULL,
    "start_time" TIMESTAMPTZ,
    "end_time" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id),
    FOREIGN KEY (stage_id) REFERENCES stages(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "act_events" (
    "id" UUID NOT NULL REFERENCES acts(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);

CREATE TABLE IF NOT EXISTS "artists" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "artist_events" (
    "id" UUID NOT NULL REFERENCES artists(id),
    "sequence" INT NOT NULL,
    "event_type" VARCHAR NOT NULL,
    "event" JSONB NOT NULL,
    "context" JSONB DEFAULT NULL,
    "recorded_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (id, sequence)
);
