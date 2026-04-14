BEGIN;

CREATE TABLE public.storage (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    "type" TEXT NOT NULL,
    location TEXT NOT NULL,
    temperature_c BIGINT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

COMMIT;
