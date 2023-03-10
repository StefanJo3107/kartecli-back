PGDMP     	                    z            BazaRMT    15.1    15.1 $               0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false                       0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false                       0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false                       1262    16398    BazaRMT    DATABASE     ?   CREATE DATABASE "BazaRMT" WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_United States.1252';
    DROP DATABASE "BazaRMT";
                postgres    false            ?            1259    16418    games    TABLE     ?   CREATE TABLE public.games (
    game_id integer NOT NULL,
    home_team_id integer NOT NULL,
    guest_team_id integer NOT NULL,
    basic_tickets integer NOT NULL,
    vip_tickets integer NOT NULL,
    date character varying NOT NULL
);
    DROP TABLE public.games;
       public         heap    postgres    false            ?            1259    16417    games_game_id_seq    SEQUENCE     ?   CREATE SEQUENCE public.games_game_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 (   DROP SEQUENCE public.games_game_id_seq;
       public          postgres    false    219                        0    0    games_game_id_seq    SEQUENCE OWNED BY     G   ALTER SEQUENCE public.games_game_id_seq OWNED BY public.games.game_id;
          public          postgres    false    218            ?            1259    16409    national_teams    TABLE     s   CREATE TABLE public.national_teams (
    national_team_id integer NOT NULL,
    name character varying NOT NULL
);
 "   DROP TABLE public.national_teams;
       public         heap    postgres    false            ?            1259    16408 #   national_teams_national_team_id_seq    SEQUENCE     ?   CREATE SEQUENCE public.national_teams_national_team_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 :   DROP SEQUENCE public.national_teams_national_team_id_seq;
       public          postgres    false    217            !           0    0 #   national_teams_national_team_id_seq    SEQUENCE OWNED BY     k   ALTER SEQUENCE public.national_teams_national_team_id_seq OWNED BY public.national_teams.national_team_id;
          public          postgres    false    216            ?            1259    16437    reservations    TABLE     ?   CREATE TABLE public.reservations (
    reservation_id integer NOT NULL,
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    basic_tickets integer NOT NULL,
    vip_tickets integer NOT NULL
);
     DROP TABLE public.reservations;
       public         heap    postgres    false            ?            1259    16436    reservations_reservation_id_seq    SEQUENCE     ?   CREATE SEQUENCE public.reservations_reservation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 6   DROP SEQUENCE public.reservations_reservation_id_seq;
       public          postgres    false    221            "           0    0    reservations_reservation_id_seq    SEQUENCE OWNED BY     c   ALTER SEQUENCE public.reservations_reservation_id_seq OWNED BY public.reservations.reservation_id;
          public          postgres    false    220            ?            1259    16400    users    TABLE     1  CREATE TABLE public.users (
    user_id integer NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    name character varying NOT NULL,
    surname character varying NOT NULL,
    identity_number character varying NOT NULL,
    email character varying NOT NULL
);
    DROP TABLE public.users;
       public         heap    postgres    false            ?            1259    16399    users_user_id_seq    SEQUENCE     ?   CREATE SEQUENCE public.users_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 (   DROP SEQUENCE public.users_user_id_seq;
       public          postgres    false    215            #           0    0    users_user_id_seq    SEQUENCE OWNED BY     G   ALTER SEQUENCE public.users_user_id_seq OWNED BY public.users.user_id;
          public          postgres    false    214            v           2604    16421    games game_id    DEFAULT     n   ALTER TABLE ONLY public.games ALTER COLUMN game_id SET DEFAULT nextval('public.games_game_id_seq'::regclass);
 <   ALTER TABLE public.games ALTER COLUMN game_id DROP DEFAULT;
       public          postgres    false    218    219    219            u           2604    16412    national_teams national_team_id    DEFAULT     ?   ALTER TABLE ONLY public.national_teams ALTER COLUMN national_team_id SET DEFAULT nextval('public.national_teams_national_team_id_seq'::regclass);
 N   ALTER TABLE public.national_teams ALTER COLUMN national_team_id DROP DEFAULT;
       public          postgres    false    216    217    217            w           2604    16440    reservations reservation_id    DEFAULT     ?   ALTER TABLE ONLY public.reservations ALTER COLUMN reservation_id SET DEFAULT nextval('public.reservations_reservation_id_seq'::regclass);
 J   ALTER TABLE public.reservations ALTER COLUMN reservation_id DROP DEFAULT;
       public          postgres    false    220    221    221            t           2604    16403    users user_id    DEFAULT     n   ALTER TABLE ONLY public.users ALTER COLUMN user_id SET DEFAULT nextval('public.users_user_id_seq'::regclass);
 <   ALTER TABLE public.users ALTER COLUMN user_id DROP DEFAULT;
       public          postgres    false    215    214    215                      0    16418    games 
   TABLE DATA           g   COPY public.games (game_id, home_team_id, guest_team_id, basic_tickets, vip_tickets, date) FROM stdin;
    public          postgres    false    219   K*                 0    16409    national_teams 
   TABLE DATA           @   COPY public.national_teams (national_team_id, name) FROM stdin;
    public          postgres    false    217   ?*                 0    16437    reservations 
   TABLE DATA           d   COPY public.reservations (reservation_id, user_id, game_id, basic_tickets, vip_tickets) FROM stdin;
    public          postgres    false    221   ?+                 0    16400    users 
   TABLE DATA           c   COPY public.users (user_id, username, password, name, surname, identity_number, email) FROM stdin;
    public          postgres    false    215   ?+       $           0    0    games_game_id_seq    SEQUENCE SET     @   SELECT pg_catalog.setval('public.games_game_id_seq', 1, false);
          public          postgres    false    218            %           0    0 #   national_teams_national_team_id_seq    SEQUENCE SET     R   SELECT pg_catalog.setval('public.national_teams_national_team_id_seq', 1, false);
          public          postgres    false    216            &           0    0    reservations_reservation_id_seq    SEQUENCE SET     N   SELECT pg_catalog.setval('public.reservations_reservation_id_seq', 1, false);
          public          postgres    false    220            '           0    0    users_user_id_seq    SEQUENCE SET     @   SELECT pg_catalog.setval('public.users_user_id_seq', 1, false);
          public          postgres    false    214            }           2606    16425    games games_pkey 
   CONSTRAINT     S   ALTER TABLE ONLY public.games
    ADD CONSTRAINT games_pkey PRIMARY KEY (game_id);
 :   ALTER TABLE ONLY public.games DROP CONSTRAINT games_pkey;
       public            postgres    false    219            {           2606    16416 "   national_teams national_teams_pkey 
   CONSTRAINT     n   ALTER TABLE ONLY public.national_teams
    ADD CONSTRAINT national_teams_pkey PRIMARY KEY (national_team_id);
 L   ALTER TABLE ONLY public.national_teams DROP CONSTRAINT national_teams_pkey;
       public            postgres    false    217                       2606    16442    reservations reservations_pkey 
   CONSTRAINT     h   ALTER TABLE ONLY public.reservations
    ADD CONSTRAINT reservations_pkey PRIMARY KEY (reservation_id);
 H   ALTER TABLE ONLY public.reservations DROP CONSTRAINT reservations_pkey;
       public            postgres    false    221            y           2606    16407    users users_pkey 
   CONSTRAINT     S   ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (user_id);
 :   ALTER TABLE ONLY public.users DROP CONSTRAINT users_pkey;
       public            postgres    false    215            ?           2606    16448    reservations fk_game    FK CONSTRAINT     ?   ALTER TABLE ONLY public.reservations
    ADD CONSTRAINT fk_game FOREIGN KEY (game_id) REFERENCES public.games(game_id) ON DELETE CASCADE;
 >   ALTER TABLE ONLY public.reservations DROP CONSTRAINT fk_game;
       public          postgres    false    219    221    3197            ?           2606    16431    games fk_guest    FK CONSTRAINT     ?   ALTER TABLE ONLY public.games
    ADD CONSTRAINT fk_guest FOREIGN KEY (guest_team_id) REFERENCES public.national_teams(national_team_id) ON DELETE CASCADE;
 8   ALTER TABLE ONLY public.games DROP CONSTRAINT fk_guest;
       public          postgres    false    217    219    3195            ?           2606    16426    games fk_home    FK CONSTRAINT     ?   ALTER TABLE ONLY public.games
    ADD CONSTRAINT fk_home FOREIGN KEY (home_team_id) REFERENCES public.national_teams(national_team_id) ON DELETE CASCADE;
 7   ALTER TABLE ONLY public.games DROP CONSTRAINT fk_home;
       public          postgres    false    3195    219    217            ?           2606    16443    reservations fk_user    FK CONSTRAINT     ?   ALTER TABLE ONLY public.reservations
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON DELETE CASCADE;
 >   ALTER TABLE ONLY public.reservations DROP CONSTRAINT fk_user;
       public          postgres    false    215    3193    221               ?   x?m??!?c0 ??r??q??+U޷G?F60C?3Kɉ+\v?B?>a???W?;?!?g?F????Y??&:??8???????|?l\??????YV??7?S?6???????????SJ_?<?         ?   x?%??
?@E??W?b?R?Ut?(???M?C;0$%?"??m??νO?:o??;`oWc??^R??qH?10?p??+8?r??N?S?x ?c?6w5?x????"R?Ҵ ??y?-G$???4)???:???j?H??? ???4.?g????5+            x?????? ? ?            x?????? ? ?     