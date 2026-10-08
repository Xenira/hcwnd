CREATE TABLE IF NOT EXISTS "events" (
    global_id      BIGSERIAL PRIMARY KEY,
    aggregate_kind TEXT                        NOT NULL,
    aggregate_id   UUID                        NOT NULL,
    sequence       BIGINT CHECK (sequence >= 0) NOT NULL,
    event_kind     TEXT                        NOT NULL,
    data           JSON                        NOT NULL,
    created_at     TIMESTAMPTZ                 NOT NULL DEFAULT NOW(),
    UNIQUE (aggregate_kind, aggregate_id, sequence)
);

CREATE INDEX idx_events_aggregate ON events (aggregate_kind, aggregate_id);

CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "sanitized_email" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TYPE "event_state" AS ENUM ('unapproved', 'live', 'rejected', 'deleted');

CREATE TABLE IF NOT EXISTS "festivals" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "start_date" DATE NOT NULL,
    "organizer_id" UUID REFERENCES users(id),
    "state" event_state NOT NULL DEFAULT 'unapproved',
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "days" (
    "id" UUID PRIMARY KEY,
    "festival_id" UUID NOT NULL REFERENCES festivals(id),
    "day_number" SMALLINT NOT NULL,
    "start_time" TIME,
    "end_time" TIME,
    "created_at" TIMESTAMPTZ NOT NULL,
    UNIQUE (festival_id, day_number)
);

CREATE TABLE IF NOT EXISTS "stages" (
    "id" UUID PRIMARY KEY,
    "festival_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (festival_id) REFERENCES festivals(id)
);

CREATE TABLE IF NOT EXISTS "sets" (
    "id" UUID PRIMARY KEY,
    "festival_id" UUID NOT NULL,
    "stage_id" UUID,
    "name" TEXT NOT NULL,
    "start_time" TIMESTAMPTZ,
    "end_time" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (festival_id) REFERENCES festivals(id),
    FOREIGN KEY (stage_id) REFERENCES stages(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "artists" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL
);
