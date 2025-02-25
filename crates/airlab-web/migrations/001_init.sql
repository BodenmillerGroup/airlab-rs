--
-- PostgreSQL database dump
--

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: clone; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.clone (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    protein_id integer NOT NULL,
    species_id integer,
    name character varying NOT NULL,
    isotype character varying,
    epitope character varying,
    is_phospho boolean DEFAULT false NOT NULL,
    is_polyclonal boolean DEFAULT false NOT NULL,
    reactivity integer[],
    application jsonb,
    is_archived boolean DEFAULT false NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.clone OWNER TO histolab_user;

--
-- Name: clone_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.clone_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.clone_id_seq OWNER TO histolab_user;

--
-- Name: clone_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.clone_id_seq OWNED BY public.clone.id;


--
-- Name: conjugate_tube_number_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.conjugate_tube_number_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.conjugate_tube_number_seq OWNER TO histolab_user;

--
-- Name: conjugate; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.conjugate (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    labeled_by integer,
    finished_by integer,
    lot_id integer NOT NULL,
    tag_id integer NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    tube_number integer DEFAULT nextval('public.conjugate_tube_number_seq'::regclass) NOT NULL,
    concentration real,
    description character varying,
    finished_at timestamp with time zone,
    is_archived boolean DEFAULT false NOT NULL,
    custom_id character varying,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.conjugate OWNER TO histolab_user;

--
-- Name: conjugate_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.conjugate_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.conjugate_id_seq OWNER TO histolab_user;

--
-- Name: conjugate_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.conjugate_id_seq OWNED BY public.conjugate.id;


--
-- Name: group; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public."group" (
    id integer NOT NULL,
    name character varying NOT NULL,
    institution character varying,
    url character varying,
    is_open boolean DEFAULT false NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    description character varying,
    location character varying(4096),
    tags character varying(64)[]
);


ALTER TABLE public."group" OWNER TO histolab_user;

--
-- Name: group_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.group_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.group_id_seq OWNER TO histolab_user;

--
-- Name: group_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.group_id_seq OWNED BY public."group".id;


--
-- Name: lot; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.lot (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    clone_id integer NOT NULL,
    provider_id integer,
    name character varying NOT NULL,
    reference character varying,
    requested_by integer,
    approved_by integer,
    ordered_by integer,
    received_by integer,
    finished_by integer,
    number character varying,
    status smallint DEFAULT 0 NOT NULL,
    purpose character varying,
    url character varying,
    price character varying,
    note character varying,
    requested_at timestamp with time zone,
    approved_at timestamp with time zone,
    ordered_at timestamp with time zone,
    received_at timestamp with time zone,
    finished_at timestamp with time zone,
    is_archived boolean DEFAULT false NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.lot OWNER TO histolab_user;

--
-- Name: lot_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.lot_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.lot_id_seq OWNER TO histolab_user;

--
-- Name: lot_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.lot_id_seq OWNED BY public.lot.id;


--
-- Name: member; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.member (
    id integer NOT NULL,
    group_id integer NOT NULL,
    user_id integer NOT NULL,
    role smallint DEFAULT 0 NOT NULL,
    all_panels boolean DEFAULT false NOT NULL,
    activation_key character varying,
    is_active boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.member OWNER TO histolab_user;

--
-- Name: member_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.member_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.member_id_seq OWNER TO histolab_user;

--
-- Name: member_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.member_id_seq OWNED BY public.member.id;


--
-- Name: migrations; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.migrations (
    id integer NOT NULL,
    "timestamp" bigint NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.migrations OWNER TO histolab_user;

--
-- Name: migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.migrations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.migrations_id_seq OWNER TO histolab_user;

--
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- Name: user; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public."user" (
    id integer NOT NULL,
    email character varying,
    name character varying,
    password character varying,
    is_active boolean DEFAULT false NOT NULL,
    is_admin boolean DEFAULT false NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    username character varying(128) DEFAULT ''::character varying NOT NULL,
    pwd character varying(256),
    reset_token character varying(256),
    pwd_salt uuid DEFAULT gen_random_uuid() NOT NULL,
    token_salt uuid DEFAULT gen_random_uuid() NOT NULL
);


ALTER TABLE public."user" OWNER TO histolab_user;

--
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.user_id_seq OWNER TO histolab_user;

--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;


--
-- Name: new_user; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.new_user (
    id integer DEFAULT nextval('public.user_id_seq'::regclass) NOT NULL,
    email character varying,
    name character varying,
    password character varying,
    is_active boolean DEFAULT false NOT NULL,
    is_admin boolean DEFAULT false NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.new_user OWNER TO histolab_user;

--
-- Name: panel; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.panel (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    name character varying,
    description character varying,
    is_fluorophore boolean DEFAULT false NOT NULL,
    is_locked boolean DEFAULT false NOT NULL,
    application integer,
    meta jsonb,
    is_archived boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.panel OWNER TO histolab_user;

--
-- Name: panel_element; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.panel_element (
    id integer NOT NULL,
    panel_id integer NOT NULL,
    conjugate_id integer NOT NULL,
    dilution_type smallint DEFAULT 0 NOT NULL,
    concentration real
);


ALTER TABLE public.panel_element OWNER TO histolab_user;

--
-- Name: panel_element_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.panel_element_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.panel_element_id_seq OWNER TO histolab_user;

--
-- Name: panel_element_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.panel_element_id_seq OWNED BY public.panel_element.id;


--
-- Name: panel_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.panel_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.panel_id_seq OWNER TO histolab_user;

--
-- Name: panel_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.panel_id_seq OWNED BY public.panel.id;


--
-- Name: protein; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.protein (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    name character varying NOT NULL,
    description character varying,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.protein OWNER TO histolab_user;

--
-- Name: protein_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.protein_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.protein_id_seq OWNER TO histolab_user;

--
-- Name: protein_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.protein_id_seq OWNED BY public.protein.id;


--
-- Name: provider; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.provider (
    id integer NOT NULL,
    group_id integer NOT NULL,
    name character varying NOT NULL,
    description character varying,
    url character varying,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.provider OWNER TO histolab_user;

--
-- Name: provider_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.provider_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.provider_id_seq OWNER TO histolab_user;

--
-- Name: provider_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.provider_id_seq OWNED BY public.provider.id;


--
-- Name: species; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.species (
    id integer NOT NULL,
    group_id integer NOT NULL,
    name character varying NOT NULL,
    acronym character varying NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.species OWNER TO histolab_user;

--
-- Name: species_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.species_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.species_id_seq OWNER TO histolab_user;

--
-- Name: species_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.species_id_seq OWNED BY public.species.id;


--
-- Name: tag; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.tag (
    id integer NOT NULL,
    group_id integer NOT NULL,
    name character varying NOT NULL,
    description character varying,
    is_metal boolean DEFAULT false NOT NULL,
    is_fluorophore boolean DEFAULT false NOT NULL,
    is_enzyme boolean DEFAULT false NOT NULL,
    is_biotin boolean DEFAULT false NOT NULL,
    is_other boolean DEFAULT false NOT NULL,
    mw smallint,
    emission smallint,
    excitation smallint,
    status smallint DEFAULT 0 NOT NULL,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.tag OWNER TO histolab_user;

--
-- Name: tag_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.tag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.tag_id_seq OWNER TO histolab_user;

--
-- Name: tag_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.tag_id_seq OWNED BY public.tag.id;


--
-- Name: validation; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.validation (
    id integer NOT NULL,
    group_id integer NOT NULL,
    created_by integer NOT NULL,
    clone_id integer NOT NULL,
    lot_id integer,
    conjugate_id integer,
    species_id integer,
    application integer NOT NULL,
    positive_control character varying,
    negative_control character varying,
    incubation_conditions character varying,
    concentration character varying,
    concentration_unit character varying,
    tissue character varying,
    fixation integer,
    fixation_notes character varying,
    notes character varying,
    status integer DEFAULT 3 NOT NULL,
    antigen_retrieval_type character varying,
    antigen_retrieval_time character varying,
    antigen_retrieval_temperature character varying,
    saponin boolean,
    saponin_concentration character varying,
    methanol_treatment boolean,
    methanol_treatment_concentration character varying,
    surface_staining boolean,
    surface_staining_concentration character varying,
    meta jsonb,
    file_id integer,
    is_archived boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.validation OWNER TO histolab_user;

--
-- Name: validation_file; Type: TABLE; Schema: public; Owner: histolab_user
--

CREATE TABLE IF NOT EXISTS public.validation_file (
    id integer NOT NULL,
    validation_id integer NOT NULL,
    created_by integer NOT NULL,
    hash character varying NOT NULL,
    size integer,
    name character varying,
    extension character varying NOT NULL,
    description character varying,
    meta jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.validation_file OWNER TO histolab_user;

--
-- Name: validation_file_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.validation_file_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.validation_file_id_seq OWNER TO histolab_user;

--
-- Name: validation_file_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.validation_file_id_seq OWNED BY public.validation_file.id;


--
-- Name: validation_id_seq; Type: SEQUENCE; Schema: public; Owner: histolab_user
--

CREATE SEQUENCE public.validation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.validation_id_seq OWNER TO histolab_user;

--
-- Name: validation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: histolab_user
--

ALTER SEQUENCE public.validation_id_seq OWNED BY public.validation.id;


--
-- Name: clone id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone ALTER COLUMN id SET DEFAULT nextval('public.clone_id_seq'::regclass);


--
-- Name: conjugate id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate ALTER COLUMN id SET DEFAULT nextval('public.conjugate_id_seq'::regclass);


--
-- Name: group id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public."group" ALTER COLUMN id SET DEFAULT nextval('public.group_id_seq'::regclass);


--
-- Name: lot id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot ALTER COLUMN id SET DEFAULT nextval('public.lot_id_seq'::regclass);


--
-- Name: member id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.member ALTER COLUMN id SET DEFAULT nextval('public.member_id_seq'::regclass);


--
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- Name: panel id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel ALTER COLUMN id SET DEFAULT nextval('public.panel_id_seq'::regclass);


--
-- Name: panel_element id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel_element ALTER COLUMN id SET DEFAULT nextval('public.panel_element_id_seq'::regclass);


--
-- Name: protein id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.protein ALTER COLUMN id SET DEFAULT nextval('public.protein_id_seq'::regclass);


--
-- Name: provider id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.provider ALTER COLUMN id SET DEFAULT nextval('public.provider_id_seq'::regclass);


--
-- Name: species id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.species ALTER COLUMN id SET DEFAULT nextval('public.species_id_seq'::regclass);


--
-- Name: tag id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.tag ALTER COLUMN id SET DEFAULT nextval('public.tag_id_seq'::regclass);


--
-- Name: user id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- Name: validation id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation ALTER COLUMN id SET DEFAULT nextval('public.validation_id_seq'::regclass);


--
-- Name: validation_file id; Type: DEFAULT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation_file ALTER COLUMN id SET DEFAULT nextval('public.validation_file_id_seq'::regclass);


--
-- Name: validation PK_03284e4f9952ce64ddc0e64bcad; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "PK_03284e4f9952ce64ddc0e64bcad" PRIMARY KEY (id);


--
-- Name: group PK_256aa0fda9b1de1a73ee0b7106b; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT "PK_256aa0fda9b1de1a73ee0b7106b" PRIMARY KEY (id);


--
-- Name: lot PK_2ba293e2165c7b93cd766c8ac9b; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "PK_2ba293e2165c7b93cd766c8ac9b" PRIMARY KEY (id);


--
-- Name: clone PK_65b56d9190eece8e03f7da9a195; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone
    ADD CONSTRAINT "PK_65b56d9190eece8e03f7da9a195" PRIMARY KEY (id);


--
-- Name: provider PK_6ab2f66d8987bf1bfdd6136a2d5; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.provider
    ADD CONSTRAINT "PK_6ab2f66d8987bf1bfdd6136a2d5" PRIMARY KEY (id);


--
-- Name: migrations PK_8c82d7f526340ab734260ea46be; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.migrations
    ADD CONSTRAINT "PK_8c82d7f526340ab734260ea46be" PRIMARY KEY (id);


--
-- Name: tag PK_8e4052373c579afc1471f526760; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT "PK_8e4052373c579afc1471f526760" PRIMARY KEY (id);


--
-- Name: protein PK_93d9ed343c2181142af016a4160; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.protein
    ADD CONSTRAINT "PK_93d9ed343c2181142af016a4160" PRIMARY KEY (id);


--
-- Name: member PK_97cbbe986ce9d14ca5894fdc072; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.member
    ADD CONSTRAINT "PK_97cbbe986ce9d14ca5894fdc072" PRIMARY KEY (id);


--
-- Name: panel_element PK_a57617cdc21e7784f57f0f2ed4f; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel_element
    ADD CONSTRAINT "PK_a57617cdc21e7784f57f0f2ed4f" PRIMARY KEY (id);


--
-- Name: species PK_ae6a87f2423ba6c25dc43c32770; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT "PK_ae6a87f2423ba6c25dc43c32770" PRIMARY KEY (id);


--
-- Name: validation_file PK_afe3d9a5082a1466473f786be02; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation_file
    ADD CONSTRAINT "PK_afe3d9a5082a1466473f786be02" PRIMARY KEY (id);


--
-- Name: panel PK_bbd5674b69f7448974aa41ab347; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel
    ADD CONSTRAINT "PK_bbd5674b69f7448974aa41ab347" PRIMARY KEY (id);


--
-- Name: user PK_cace4a159ff9f2512dd42373760; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT "PK_cace4a159ff9f2512dd42373760" PRIMARY KEY (id);


--
-- Name: conjugate PK_f586569a204bbd9c0780d555429; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "PK_f586569a204bbd9c0780d555429" PRIMARY KEY (id);


--
-- Name: member UQ_member_group_id_and_user_id; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.member
    ADD CONSTRAINT "UQ_member_group_id_and_user_id" UNIQUE (group_id, user_id);


--
-- Name: panel_element UQ_panel_element_panel_id_and_conjugate_id; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel_element
    ADD CONSTRAINT "UQ_panel_element_panel_id_and_conjugate_id" UNIQUE (panel_id, conjugate_id);


--
-- Name: provider UQ_provider_group_id_and_name; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.provider
    ADD CONSTRAINT "UQ_provider_group_id_and_name" UNIQUE (group_id, name);


--
-- Name: species UQ_species_group_id_and_acronym; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT "UQ_species_group_id_and_acronym" UNIQUE (group_id, acronym);


--
-- Name: species UQ_species_group_id_and_name; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT "UQ_species_group_id_and_name" UNIQUE (group_id, name);


--
-- Name: tag UQ_tag_group_id_and_name_and_mw; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT "UQ_tag_group_id_and_name_and_mw" UNIQUE (group_id, name, mw);


--
-- Name: user UQ_user_email; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT "UQ_user_email" UNIQUE (email);


--
-- Name: new_user new_user_email_key; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.new_user
    ADD CONSTRAINT new_user_email_key UNIQUE (email);


--
-- Name: new_user new_user_pkey; Type: CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.new_user
    ADD CONSTRAINT new_user_pkey PRIMARY KEY (id);


--
-- Name: IDX_clone_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_clone_created_by" ON public.clone USING btree (created_by);


--
-- Name: IDX_clone_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_clone_group_id" ON public.clone USING btree (group_id);


--
-- Name: IDX_clone_name; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_clone_name" ON public.clone USING btree (name);


--
-- Name: IDX_clone_protein_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_clone_protein_id" ON public.clone USING btree (protein_id);


--
-- Name: IDX_conjugate_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_created_by" ON public.conjugate USING btree (created_by);


--
-- Name: IDX_conjugate_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_group_id" ON public.conjugate USING btree (group_id);


--
-- Name: IDX_conjugate_lot_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_lot_id" ON public.conjugate USING btree (lot_id);


--
-- Name: IDX_conjugate_status; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_status" ON public.conjugate USING btree (status);


--
-- Name: IDX_conjugate_tag_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_tag_id" ON public.conjugate USING btree (tag_id);


--
-- Name: IDX_conjugate_tube_number; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_conjugate_tube_number" ON public.conjugate USING btree (tube_number);


--
-- Name: IDX_lot_clone_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_lot_clone_id" ON public.lot USING btree (clone_id);


--
-- Name: IDX_lot_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_lot_created_by" ON public.lot USING btree (created_by);


--
-- Name: IDX_lot_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_lot_group_id" ON public.lot USING btree (group_id);


--
-- Name: IDX_lot_provider_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_lot_provider_id" ON public.lot USING btree (provider_id);


--
-- Name: IDX_lot_status; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_lot_status" ON public.lot USING btree (status);


--
-- Name: IDX_member_activation_key; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_member_activation_key" ON public.member USING btree (activation_key);


--
-- Name: IDX_member_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_member_group_id" ON public.member USING btree (group_id);


--
-- Name: IDX_member_is_active; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_member_is_active" ON public.member USING btree (is_active);


--
-- Name: IDX_member_user_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_member_user_id" ON public.member USING btree (user_id);


--
-- Name: IDX_panel_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_panel_created_by" ON public.panel USING btree (created_by);


--
-- Name: IDX_panel_element_conjugate_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_panel_element_conjugate_id" ON public.panel_element USING btree (conjugate_id);


--
-- Name: IDX_panel_element_panel_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_panel_element_panel_id" ON public.panel_element USING btree (panel_id);


--
-- Name: IDX_panel_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_panel_group_id" ON public.panel USING btree (group_id);


--
-- Name: IDX_protein_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_protein_created_by" ON public.protein USING btree (created_by);


--
-- Name: IDX_protein_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_protein_group_id" ON public.protein USING btree (group_id);


--
-- Name: IDX_protein_name; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_protein_name" ON public.protein USING btree (name);


--
-- Name: IDX_provider_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_provider_group_id" ON public.provider USING btree (group_id);


--
-- Name: IDX_provider_name; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_provider_name" ON public.provider USING btree (name);


--
-- Name: IDX_species_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_species_group_id" ON public.species USING btree (group_id);


--
-- Name: IDX_tag_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_tag_group_id" ON public.tag USING btree (group_id);


--
-- Name: IDX_user_email; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_user_email" ON public."user" USING btree (email);


--
-- Name: IDX_user_is_active; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_user_is_active" ON public."user" USING btree (is_active);


--
-- Name: IDX_validation_application; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_application" ON public.validation USING btree (application);


--
-- Name: IDX_validation_clone_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_clone_id" ON public.validation USING btree (clone_id);


--
-- Name: IDX_validation_conjugate_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_conjugate_id" ON public.validation USING btree (conjugate_id);


--
-- Name: IDX_validation_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_created_by" ON public.validation USING btree (created_by);


--
-- Name: IDX_validation_file_created_by; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_file_created_by" ON public.validation_file USING btree (created_by);


--
-- Name: IDX_validation_file_hash; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_file_hash" ON public.validation_file USING btree (hash);


--
-- Name: IDX_validation_file_validation_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_file_validation_id" ON public.validation_file USING btree (validation_id);


--
-- Name: IDX_validation_group_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_group_id" ON public.validation USING btree (group_id);


--
-- Name: IDX_validation_lot_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_lot_id" ON public.validation USING btree (lot_id);


--
-- Name: IDX_validation_species_id; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_species_id" ON public.validation USING btree (species_id);


--
-- Name: IDX_validation_status; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS "IDX_validation_status" ON public.validation USING btree (status);


--
-- Name: new_user_email_idx; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS new_user_email_idx ON public.new_user USING btree (email);


--
-- Name: new_user_is_active_idx; Type: INDEX; Schema: public; Owner: histolab_user
--

CREATE INDEX IF NOT EXISTS new_user_is_active_idx ON public.new_user USING btree (is_active);


--
-- Name: clone FK_clone_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone
    ADD CONSTRAINT "FK_clone_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: clone FK_clone_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone
    ADD CONSTRAINT "FK_clone_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: clone FK_clone_to_protein; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone
    ADD CONSTRAINT "FK_clone_to_protein" FOREIGN KEY (protein_id) REFERENCES public.protein(id) ON DELETE CASCADE;


--
-- Name: clone FK_clone_to_species; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.clone
    ADD CONSTRAINT "FK_clone_to_species" FOREIGN KEY (species_id) REFERENCES public.species(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_finished_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_finished_by_to_member" FOREIGN KEY (finished_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_labeled_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_labeled_by_to_member" FOREIGN KEY (labeled_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_to_lot; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_to_lot" FOREIGN KEY (lot_id) REFERENCES public.lot(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: conjugate FK_conjugate_to_tag; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.conjugate
    ADD CONSTRAINT "FK_conjugate_to_tag" FOREIGN KEY (tag_id) REFERENCES public.tag(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_approved_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_approved_by_to_member" FOREIGN KEY (approved_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_finished_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_finished_by_to_member" FOREIGN KEY (finished_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_ordered_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_ordered_by_to_member" FOREIGN KEY (ordered_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_received_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_received_by_to_member" FOREIGN KEY (received_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_requested_by_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_requested_by_to_member" FOREIGN KEY (requested_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_to_clone; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_to_clone" FOREIGN KEY (clone_id) REFERENCES public.clone(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: lot FK_lot_to_provider; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.lot
    ADD CONSTRAINT "FK_lot_to_provider" FOREIGN KEY (provider_id) REFERENCES public.provider(id) ON DELETE CASCADE;


--
-- Name: member FK_member_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.member
    ADD CONSTRAINT "FK_member_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: member FK_member_to_user; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.member
    ADD CONSTRAINT "FK_member_to_user" FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE;


--
-- Name: panel_element FK_panel_element_to_conjugate; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel_element
    ADD CONSTRAINT "FK_panel_element_to_conjugate" FOREIGN KEY (conjugate_id) REFERENCES public.conjugate(id) ON DELETE CASCADE;


--
-- Name: panel_element FK_panel_element_to_panel; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel_element
    ADD CONSTRAINT "FK_panel_element_to_panel" FOREIGN KEY (panel_id) REFERENCES public.panel(id) ON DELETE CASCADE;


--
-- Name: panel FK_panel_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel
    ADD CONSTRAINT "FK_panel_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: panel FK_panel_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.panel
    ADD CONSTRAINT "FK_panel_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: protein FK_protein_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.protein
    ADD CONSTRAINT "FK_protein_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: protein FK_protein_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.protein
    ADD CONSTRAINT "FK_protein_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: provider FK_provider_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.provider
    ADD CONSTRAINT "FK_provider_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: species FK_species_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT "FK_species_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: tag FK_tag_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.tag
    ADD CONSTRAINT "FK_tag_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: validation_file FK_validation_file_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation_file
    ADD CONSTRAINT "FK_validation_file_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: validation_file FK_validation_file_to_validation; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation_file
    ADD CONSTRAINT "FK_validation_file_to_validation" FOREIGN KEY (validation_id) REFERENCES public.validation(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_clone; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_clone" FOREIGN KEY (clone_id) REFERENCES public.clone(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_conjugate; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_conjugate" FOREIGN KEY (conjugate_id) REFERENCES public.conjugate(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_group; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_group" FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_lot; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_lot" FOREIGN KEY (lot_id) REFERENCES public.lot(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_member; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_member" FOREIGN KEY (created_by) REFERENCES public.member(id) ON DELETE CASCADE;


--
-- Name: validation FK_validation_to_species; Type: FK CONSTRAINT; Schema: public; Owner: histolab_user
--

ALTER TABLE ONLY public.validation
    ADD CONSTRAINT "FK_validation_to_species" FOREIGN KEY (species_id) REFERENCES public.species(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

