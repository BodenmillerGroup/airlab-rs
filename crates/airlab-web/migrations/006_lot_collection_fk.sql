BEGIN;

ALTER TABLE public.lot
    ADD COLUMN IF NOT EXISTS collection_id BIGINT;

CREATE INDEX IF NOT EXISTS idx_lot_collection_id
    ON public.lot (collection_id);

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_to_collection"
    FOREIGN KEY (collection_id)
    REFERENCES public.collection(id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT;

COMMIT;
