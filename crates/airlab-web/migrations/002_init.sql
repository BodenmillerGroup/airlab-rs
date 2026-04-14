BEGIN;

SET LOCAL lock_timeout = '10s';
SET LOCAL statement_timeout = '0';

-- drop constraints

ALTER TABLE ONLY public.clone DROP CONSTRAINT "FK_clone_to_group";
ALTER TABLE ONLY public.clone DROP CONSTRAINT "FK_clone_to_member";
ALTER TABLE ONLY public.clone DROP CONSTRAINT "FK_clone_to_protein";
ALTER TABLE ONLY public.clone DROP CONSTRAINT "FK_clone_to_species";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_finished_by_to_member";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_labeled_by_to_member";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_to_group";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_to_lot";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_to_member";
ALTER TABLE ONLY public.conjugate DROP CONSTRAINT "FK_conjugate_to_tag";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_approved_by_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_finished_by_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_ordered_by_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_received_by_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_requested_by_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_to_clone";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_to_group";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_to_member";
ALTER TABLE ONLY public.lot DROP CONSTRAINT "FK_lot_to_provider";
ALTER TABLE ONLY public.member DROP CONSTRAINT "FK_member_to_group";
ALTER TABLE ONLY public.member DROP CONSTRAINT "FK_member_to_user";
ALTER TABLE ONLY public.panel_element DROP CONSTRAINT "FK_panel_element_to_conjugate";
ALTER TABLE ONLY public.panel_element DROP CONSTRAINT "FK_panel_element_to_panel";
ALTER TABLE ONLY public.panel DROP CONSTRAINT "FK_panel_to_group";
ALTER TABLE ONLY public.panel DROP CONSTRAINT "FK_panel_to_member";
ALTER TABLE ONLY public.protein DROP CONSTRAINT "FK_protein_to_group";
ALTER TABLE ONLY public.protein DROP CONSTRAINT "FK_protein_to_member";
ALTER TABLE ONLY public.provider DROP CONSTRAINT "FK_provider_to_group";
ALTER TABLE ONLY public.species DROP CONSTRAINT "FK_species_to_group";
ALTER TABLE ONLY public.tag DROP CONSTRAINT "FK_tag_to_group";
ALTER TABLE ONLY public.validation_file DROP CONSTRAINT "FK_validation_file_to_member";
ALTER TABLE ONLY public.validation_file DROP CONSTRAINT "FK_validation_file_to_validation";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_clone";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_conjugate";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_group";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_lot";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_member";
ALTER TABLE ONLY public.validation DROP CONSTRAINT "FK_validation_to_species";

-- integer to bigint


ALTER TABLE public.clone
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN protein_id TYPE bigint,
  ALTER COLUMN species_id TYPE bigint,
  ALTER COLUMN reactivity TYPE bigint[];


ALTER TABLE public.conjugate
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN labeled_by TYPE bigint,
  ALTER COLUMN finished_by TYPE bigint,
  ALTER COLUMN lot_id TYPE bigint,
  ALTER COLUMN tag_id TYPE bigint,
  ALTER COLUMN concentration TYPE double precision,
  ALTER COLUMN tube_number TYPE bigint,
  ALTER COLUMN status TYPE bigint;

ALTER TABLE public."group"
  ALTER COLUMN id TYPE bigint;

ALTER TABLE public.lot
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN clone_id TYPE bigint,
  ALTER COLUMN provider_id TYPE bigint,
  ALTER COLUMN requested_by TYPE bigint,
  ALTER COLUMN approved_by TYPE bigint,
  ALTER COLUMN ordered_by TYPE bigint,
  ALTER COLUMN received_by TYPE bigint,
  ALTER COLUMN finished_by TYPE bigint,
  ALTER COLUMN status TYPE bigint;

ALTER TABLE public.member
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN user_id TYPE bigint,
  ALTER COLUMN role TYPE bigint;

ALTER TABLE public.panel
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN application TYPE bigint;

ALTER TABLE public.panel_element
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN panel_id TYPE bigint,
  ALTER COLUMN conjugate_id TYPE bigint,
  ALTER COLUMN dilution_type TYPE bigint;

ALTER TABLE public.protein
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint;

ALTER TABLE public.provider
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint;

ALTER TABLE public.species
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint;

ALTER TABLE public.tag
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN mw TYPE bigint,
  ALTER COLUMN emission TYPE bigint,
  ALTER COLUMN excitation TYPE bigint,
  ALTER COLUMN status TYPE bigint;

ALTER TABLE public."user"
  ALTER COLUMN id TYPE bigint;

ALTER TABLE public.validation
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN group_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN clone_id TYPE bigint,
  ALTER COLUMN lot_id TYPE bigint,
  ALTER COLUMN conjugate_id TYPE bigint,
  ALTER COLUMN species_id TYPE bigint,
  ALTER COLUMN application TYPE bigint,
  ALTER COLUMN fixation TYPE bigint,
  ALTER COLUMN status TYPE bigint,
  ALTER COLUMN file_id TYPE bigint;

ALTER TABLE public.validation_file
  ALTER COLUMN id TYPE bigint,
  ALTER COLUMN validation_id TYPE bigint,
  ALTER COLUMN created_by TYPE bigint,
  ALTER COLUMN size TYPE bigint;

-- mid/cid

ALTER TABLE "validation_file" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "validation_file" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "validation_file" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "validation_file" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "validation" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "validation" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "validation" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "validation" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "panel_element" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "panel_element" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "panel_element" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "panel_element" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "panel" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "panel" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "panel" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "panel" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "lot" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "lot" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "lot" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "lot" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "conjugate" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "conjugate" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "conjugate" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "conjugate" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "clone" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "clone" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "clone" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "clone" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "tag" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "tag" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "tag" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "tag" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "species" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "species" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "species" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "species" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "provider" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "provider" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "provider" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "provider" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "protein" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "protein" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "protein" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "protein" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "member" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "member" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "member" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "member" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "group" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "group" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "group" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "group" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "user" ADD COLUMN mfa_enabled bool NOT NULL default false;
ALTER TABLE "user" ADD COLUMN mfa_secret character varying NOT NULL default '';
ALTER TABLE "user" ADD COLUMN cid bigint NOT NULL default 1;
ALTER TABLE "user" ADD COLUMN ctime timestamp WITH time zone DEFAULT now() NOT NULL;
ALTER TABLE "user" ADD COLUMN mid bigint NOT NULL default 1;
ALTER TABLE "user" ADD COLUMN mtime timestamp WITH time zone DEFAULT now() NOT NULL;

-- add constraints back

ALTER TABLE ONLY public.clone ADD CONSTRAINT "FK_clone_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.clone ADD CONSTRAINT "FK_clone_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.clone ADD CONSTRAINT "FK_clone_to_protein" FOREIGN KEY (protein_id) REFERENCES public.protein(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.clone ADD CONSTRAINT "FK_clone_to_species" FOREIGN KEY (species_id) REFERENCES public.species(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_finished_by_to_member" FOREIGN KEY (finished_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_labeled_by_to_member" FOREIGN KEY (labeled_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_to_lot" FOREIGN KEY (lot_id) REFERENCES public.lot(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.conjugate ADD CONSTRAINT "FK_conjugate_to_tag" FOREIGN KEY (tag_id) REFERENCES public.tag(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_approved_by_to_member" FOREIGN KEY (approved_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_finished_by_to_member" FOREIGN KEY (finished_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_ordered_by_to_member" FOREIGN KEY (ordered_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_received_by_to_member" FOREIGN KEY (received_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_requested_by_to_member" FOREIGN KEY (requested_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_to_clone" FOREIGN KEY (clone_id) REFERENCES public.clone(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.lot ADD CONSTRAINT "FK_lot_to_provider" FOREIGN KEY (provider_id) REFERENCES public.provider(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.member ADD CONSTRAINT "FK_member_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.member ADD CONSTRAINT "FK_member_to_user" FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.panel_element ADD CONSTRAINT "FK_panel_element_to_conjugate" FOREIGN KEY (conjugate_id) REFERENCES public.conjugate(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.panel_element ADD CONSTRAINT "FK_panel_element_to_panel" FOREIGN KEY (panel_id) REFERENCES public.panel(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.panel ADD CONSTRAINT "FK_panel_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.panel ADD CONSTRAINT "FK_panel_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.protein ADD CONSTRAINT "FK_protein_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.protein ADD CONSTRAINT "FK_protein_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.provider ADD CONSTRAINT "FK_provider_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.species ADD CONSTRAINT "FK_species_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.tag ADD CONSTRAINT "FK_tag_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation_file ADD CONSTRAINT "FK_validation_file_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation_file ADD CONSTRAINT "FK_validation_file_to_validation" FOREIGN KEY (validation_id) REFERENCES public.validation(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_clone" FOREIGN KEY (clone_id) REFERENCES public.clone(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_conjugate" FOREIGN KEY (conjugate_id) REFERENCES public.conjugate(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_lot" FOREIGN KEY (lot_id) REFERENCES public.lot(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE RESTRICT;
ALTER TABLE ONLY public.validation ADD CONSTRAINT "FK_validation_to_species" FOREIGN KEY (species_id) REFERENCES public.species(id) ON DELETE RESTRICT;

COMMIT;
