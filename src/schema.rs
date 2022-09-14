table! {
    carousel (id_carousel) {
        id_carousel -> Int4,
        title_carousel_1 -> Text,
        title_carousel_2 -> Text,
        title_carousel_3 -> Text,
        p_carousel_1 -> Text,
        p_carousel_2 -> Text,
        p_carousel_3 -> Text,
        img_carousel_1 -> Text,
        img_carousel_2 -> Text,
        img_carousel_3 -> Text,
        href_carousel_1 -> Text,
        href_carousel_2 -> Text,
        href_carousel_3 -> Text,
    }
}

table! {
    layoutpost (id_layout_post) {
        id_layout_post -> Int4,
        title_layout_post -> Varchar,
        p_p_layout_post -> Text,
        img_layout_post_1 -> Text,
        img_layout_post_2 -> Text,
        title_button_layout_post_1 -> Varchar,
        href_layout_post_1 -> Text,
    }
}

table! {
    layoutprofile (id_layout_profile) {
        id_layout_profile -> Int4,
        title_layout_profile -> Varchar,
        p_p_layout_profile -> Text,
        img_layout_profile -> Text,
        title_button_layout_profile_1 -> Varchar,
        title_button_layout_profile_2 -> Varchar,
        href_layout_profile_1 -> Text,
        href_layout_profile_2 -> Text,
    }
}

table! {
    layoutservice (id_layout_service) {
        id_layout_service -> Int4,
        title_layout_service -> Varchar,
        p_p_layout_service -> Text,
        img_layout_service -> Text,
        title_button_layout_service_1 -> Varchar,
        title_button_layout_service_2 -> Varchar,
        href_layout_service_1 -> Text,
        href_layout_service_2 -> Text,
    }
}

table! {
    msg (email) {
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        country -> Varchar,
        message -> Text,
    }
}

table! {
    p_contact (id_p_contact) {
        id_p_contact -> Int4,
        title_p_contact -> Text,
        p_p_contact_1 -> Text,
        p_p_contact_2 -> Text,
        p_p_contact_3 -> Text,
        title_button_contact_1 -> Varchar,
        title_button_contact_2 -> Varchar,
        title_button_contact_3 -> Varchar,
        href_contact_1 -> Text,
        href_contact_2 -> Text,
        href_contact_3 -> Text,
    }
}

table! {
    pindex (id_p_index) {
        id_p_index -> Int4,
        title_p_index -> Text,
        sub_title_p_index -> Text,
        p_p_index_1 -> Text,
        p_p_index_2 -> Text,
        p_p_index_3 -> Text,
        img_p_index -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Text,
        body -> Text,
        img -> Text,
        date -> Text,
        published -> Bool,
    }
}

table! {
    pprofile (id_p_profile) {
        id_p_profile -> Int4,
        title_p_profile -> Text,
        sub_title_p_profile -> Text,
        p_p_profile_1 -> Text,
        p_p_profile_2 -> Text,
        p_p_profile_3 -> Text,
        img_p_profile -> Text,
    }
}

table! {
    profiles (id_profile) {
        id_profile -> Int4,
        name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        git_hub -> Varchar,
        phone -> Varchar,
        address -> Varchar,
        city -> Varchar,
        country -> Varchar,
        text -> Text,
        published_profile -> Bool,
    }
}

table! {
    pservice (id_p_service) {
        id_p_service -> Int4,
        title_p_service -> Text,
        sub_title_p_service -> Text,
        p_p_service_1 -> Text,
        p_p_service_2 -> Text,
        p_p_service_3 -> Text,
        img_p_service -> Text,
    }
}

table! {
    services (id_service) {
        id_service -> Int4,
        name -> Varchar,
        description -> Text,
        img_service_1 -> Text,
        img_service_2 -> Text,
        img_service_3 -> Text,
        published_service -> Bool,
    }
}

table! {
    skills (id_skill) {
        id_skill -> Int4,
        title_skill -> Text,
        name_skill -> Varchar,
        level_skill -> Varchar,
        type_skill -> Varchar,
        img_skill -> Text,
        img_perfil_skill -> Text,
    }
}

table! {
    socials (id_social) {
        id_social -> Int4,
        name -> Varchar,
        telegram -> Varchar,
        linkedin -> Varchar,
        github -> Varchar,
        instagram -> Varchar,
        whatsapp -> Varchar,
        twitter -> Varchar,
        published_social -> Bool,
    }
}

table! {
    symbol (id_symbol) {
        id_symbol -> Int4,
        title_symbol -> Text,
        p_symbol -> Text,
        img_symbol -> Text,
        href_symbol -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    carousel,
    layoutpost,
    layoutprofile,
    layoutservice,
    msg,
    p_contact,
    pindex,
    posts,
    pprofile,
    profiles,
    pservice,
    services,
    skills,
    socials,
    symbol,
);
