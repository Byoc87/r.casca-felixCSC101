--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(50),
    eid integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	mkarim@gmail.com	8055089112	102	5
111	Lilian Jaiya	43	ljaiye@gmail.com	8055185341	100	3
112	Arthur	50	amuas@gmail.com	7055282813	107	10
113	Phillip Akonjo	41	pakonjo@gmail.com	70552823413	100	2
114	Marylene Mapa	33	mmapa@gmail.com	80657823413	120	5
115	Oghenero Agor	50	oagor@gmail.com	9065982615	117	11
116	Adams Bree	33	abree@gmail.com	9065952618	102	1
117	Okafor Mathias	45	omathias@gmail.com	8135942618	120	10
118	Samson Adeleke	65	sadeleke@gmail.com	8135945614	117	11
119	Lawal Tamire	35	ltamire@gmail.com	8135945632	107	5
120	James Job	44	jjob@gmail.com	7035925335	100	8
121	Matthew Jakande	21	mjakande@gmail.com	7034525336	120	2
122	Jimilia Adegboye	20	jadegboye@gmail.com	7037525098	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

