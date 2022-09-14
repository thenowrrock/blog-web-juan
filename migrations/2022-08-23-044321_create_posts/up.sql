-- Your SQL goes here
--UP se encarga de crear y actualizar la tabla de la base de datos de acuerdo a la estructura de la tabla 
--que se encuentra en el archivo up.sql 
--Create migraciones de la tabla posts



--Create migraciones de la tabla carousel index.html--
CREATE TABLE carousel(
    id_carousel serial PRIMARY KEY ,
    title_carousel_1 TEXT NOT NULL,
    title_carousel_2 TEXT NOT NULL,
    title_carousel_3 TEXT NOT NULL,
    p_carousel_1 TEXT NOT NULL,
    p_carousel_2 TEXT NOT NULL,
    p_carousel_3 TEXT NOT NULL,
    img_carousel_1 TEXT NOT NULL,
    img_carousel_2 TEXT NOT NULL,
    img_carousel_3 TEXT NOT NULL,
    href_carousel_1 TEXT NOT NULL,
    href_carousel_2 TEXT NOT NULL,
    href_carousel_3 TEXT NOT NULL
);


CREATE TABLE symbol( 
    id_symbol serial PRIMARY KEY ,
    title_symbol TEXT NOT NULL,
    p_symbol TEXT NOT NULL,
    img_symbol TEXT NOT NULL,
    href_symbol TEXT NOT NULL
);

CREATE TABLE pindex(
    id_p_index serial PRIMARY KEY ,
    title_p_index TEXT NOT NULL,
    sub_title_p_index TEXT NOT NULL,
    p_p_index_1 TEXT NOT NULL,
    p_p_index_2 TEXT NOT NULL,
    p_p_index_3 TEXT NOT NULL,
    img_p_index TEXT NOT NULL
);

--Create migraciones de la tabla services service.html--
CREATE TABLE services(
    id_service serial PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    img_service_1 TEXT NOT NULL,
    img_service_2 TEXT NOT NULL,
    img_service_3 TEXT NOT NULL,
    published_service BOOLEAN NOT NULL DEFAULT 'f'
);


CREATE TABLE Layoutservice(
    id_layout_service serial PRIMARY KEY ,
    title_layout_service VARCHAR(255) NOT NULL,
    p_p_layout_service TEXT NOT NULL,
    img_layout_service TEXT NOT NULL ,
    title_button_layout_service_1 VARCHAR(255) NOT NULL,
    title_button_layout_service_2 VARCHAR(255) NOT NULL,
    href_layout_service_1 TEXT NOT NULL,
    href_layout_service_2 TEXT NOT NULL
);

CREATE TABLE Pservice(
    id_p_service serial PRIMARY KEY ,
    title_p_service TEXT NOT NULL,
    sub_title_p_service TEXT NOT NULL,
    p_p_service_1 TEXT NOT NULL,
    p_p_service_2 TEXT NOT NULL,
    p_p_service_3 TEXT NOT NULL,
    img_p_service TEXT NOT NULL
);

--Create migraciones de la tabla profile profile.html--
CREATE TABLE Layoutprofile(
    id_layout_profile serial PRIMARY KEY ,
    title_layout_profile VARCHAR(255) NOT NULL,
    p_p_layout_profile TEXT NOT NULL,
    img_layout_profile TEXT NOT NULL ,
    title_button_layout_profile_1 VARCHAR(255) NOT NULL,
    title_button_layout_profile_2 VARCHAR(255) NOT NULL,
    href_layout_profile_1 TEXT NOT NULL,
    href_layout_profile_2 TEXT NOT NULL
);

CREATE TABLE Pprofile(
    id_p_profile serial PRIMARY KEY ,
    title_p_profile TEXT NOT NULL,
    sub_title_p_profile TEXT NOT NULL,
    p_p_profile_1 TEXT NOT NULL,
    p_p_profile_2 TEXT NOT NULL,
    p_p_profile_3 TEXT NOT NULL,
    img_p_profile TEXT NOT NULL
);

CREATE TABLE profiles(
    id_profile serial PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    git_hub VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    text TEXT NOT NULL,
    published_profile BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE skills(
    id_skill serial PRIMARY KEY ,
    title_skill TEXT NOT NULL,
    name_skill VARCHAR(255) NOT NULL,
    level_skill VARCHAR(255) NOT NULL,
    type_skill VARCHAR(255) NOT NULL,
    img_skill TEXT NOT NULL,
    img_perfil_skill TEXT NOT NULL
);


--create migraciones de la tabla posts post.html--
CREATE TABLE posts(
    id serial PRIMARY KEY ,
    title VARCHAR(255) NOT NULL,
    slug TEXT NOT NULL,
    body TEXT NOT NULL,
    img TEXT NOT NULL,
    date TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE Layoutpost(
    id_layout_post serial PRIMARY KEY ,
    title_layout_post VARCHAR(255) NOT NULL,
    p_p_layout_post TEXT NOT NULL,
    img_layout_post_1 TEXT NOT NULL ,
    img_layout_post_2 TEXT NOT NULL ,
    title_button_layout_post_1 VARCHAR(255) NOT NULL,
    href_layout_post_1 TEXT NOT NULL
);

--Create migraciones para contact
CREATE TABLE socials(
    id_social serial PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    telegram VARCHAR(255) NOT NULL,
    linkedin VARCHAR(255) NOT NULL,
    github VARCHAR(255) NOT NULL,
    instagram VARCHAR(255) NOT NULL,
    whatsapp VARCHAR(255) NOT NULL,
    twitter VARCHAR(255) NOT NULL,
    published_social BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE p_contact(
    id_p_contact serial PRIMARY KEY ,
    title_p_contact TEXT NOT NULL,
    p_p_contact_1 TEXT NOT NULL,
    p_p_contact_2 TEXT NOT NULL,
    p_p_contact_3 TEXT NOT NULL,
    title_button_contact_1 VARCHAR(255) NOT NULL,
    title_button_contact_2 VARCHAR(255) NOT NULL,
    title_button_contact_3 VARCHAR(255) NOT NULL,
    href_contact_1 TEXT NOT NULL ,
    href_contact_2 TEXT NOT NULL ,
    href_contact_3 TEXT NOT NULL 
);

CREATE TABLE Msg(
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL PRIMARY KEY,
    country VARCHAR(255) NOT NULL,
    message TEXT NOT NULL
);
