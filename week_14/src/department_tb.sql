--
-- PostgreSQL database dump
--

\restrict MGOlCLo1yOVPP5eX2Say4MvrrVhVvdwf4TZeqJuCOkgIlAdfaqFYq78rPv36gkb

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
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict MGOlCLo1yOVPP5eX2Say4MvrrVhVvdwf4TZeqJuCOkgIlAdfaqFYq78rPv36gkb

