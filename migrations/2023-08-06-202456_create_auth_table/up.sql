-- Your SQL goes here
CREATE TABLE "auth"(
    "id" SERIAL PRIMARY KEY,
    "user_id" INTEGER UNIQUE NOT NULL,
    "pin" VARCHAR(6) NOT NULL,
    "tries" INTEGER NOT NULL DEFAULT 0,
    "expires_at" TIMESTAMPTZ NOT NULL DEFAULT (NOW() + interval '30 minutes'),
    "locked_until" TIMESTAMPTZ DEFAULT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT
);
