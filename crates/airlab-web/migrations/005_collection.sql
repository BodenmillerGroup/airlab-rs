BEGIN;

CREATE TABLE public.collection (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_collection_created_by
    ON public.collection (created_by);

ALTER TABLE public.collection
    ADD CONSTRAINT fk_collection_created_by_member
    FOREIGN KEY (created_by)
    REFERENCES public.member(id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT;

COMMIT;
