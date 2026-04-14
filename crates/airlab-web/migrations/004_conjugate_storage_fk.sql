BEGIN;

ALTER TABLE public.conjugate
    ADD COLUMN IF NOT EXISTS storage_id BIGINT NULL;

CREATE INDEX IF NOT EXISTS idx_conjugate_storage_id
    ON public.conjugate (storage_id);

ALTER TABLE public.conjugate
    DROP CONSTRAINT IF EXISTS conjugate_storage_id_fkey;

ALTER TABLE public.conjugate
    ADD CONSTRAINT conjugate_storage_id_fkey
    FOREIGN KEY (storage_id)
    REFERENCES public.storage(id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT;

COMMIT;
