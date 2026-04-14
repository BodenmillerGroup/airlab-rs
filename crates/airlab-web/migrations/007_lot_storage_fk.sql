BEGIN;

ALTER TABLE public.lot
    ADD COLUMN IF NOT EXISTS storage_id BIGINT NULL;

CREATE INDEX IF NOT EXISTS idx_lot_storage_id
    ON public.lot (storage_id);

ALTER TABLE public.lot
    DROP CONSTRAINT IF EXISTS lot_storage_id_fkey;

ALTER TABLE public.lot
    ADD CONSTRAINT lot_storage_id_fkey
    FOREIGN KEY (storage_id)
    REFERENCES public.storage(id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT;

COMMIT;
