--
-- PostgreSQL database dump
--

\restrict 2iTQVj77isIztlehpafMD3qQ2WfFVUKDepFF8Sv3QoXKcNFtmFk0u67cgxlhgXt

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name character(50) CONSTRAINT employees_ename_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_sal real CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
97	Dankade Aminat                                    	5	550000	40	08023089832
100	Mustapha Ali                                      	3	175000	32	08063285831
102	Mankide Mary                                      	2	450000	55	09023688832
104	Kuti lawal                                        	1	750000	35	08053189131
107	Alokwe Martin                                     	7	380000	48	09023487830
108	Josiah joshua                                     	1	120000	30	07061045862
117	Suleman Ajayi                                     	3	800000	50	0802289842
120	Adeleke Jane                                      	4	200000	38	07030089811
122	Osahon Mark                                       	6	320000	44	0914568942
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict 2iTQVj77isIztlehpafMD3qQ2WfFVUKDepFF8Sv3QoXKcNFtmFk0u67cgxlhgXt

