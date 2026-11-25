--
-- PostgreSQL database dump
--

\restrict YHgqX1PqEHQ3O4nPSJ0zmKvX7Xvzs28KCiRlG6EKqRBFQr1T3RnunmxBe4tTVG0

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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname character(50) NOT NULL,
    dlocation character(50) NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character varying(50) NOT NULL,
    pduration character(20) NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION                                    	IKEJA                                             	44
101	2	ACCOUNT                                           	EGBEDA                                            	11
100	3	PACKAGING                                         	AJAH                                              	44
120	5	RESEARCH                                          	V.I                                               	33
97	5	ACCOUNT                                           	MAGADO                                            	22
122	6	OPERATIONS                                        	MILE 2                                            	44
107	7	PACKAGING                                         	KETU                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 MONTHS            	102
22	B	14 MONTHS           	97
33	C	16 MONTHS           	120
44	D	25 MONTHS           	108
55	E	9 MONTHS            	107
\.


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
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

\unrestrict YHgqX1PqEHQ3O4nPSJ0zmKvX7Xvzs28KCiRlG6EKqRBFQr1T3RnunmxBe4tTVG0

