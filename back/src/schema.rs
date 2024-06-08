// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_type_variation"))]
    pub struct ItemTypeVariation;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_permission"))]
    pub struct ListPermission;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "notif_type"))]
    pub struct NotifType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "priority_type"))]
    pub struct PriorityType;
}

diesel::table! {
    allowed_items (list_type_id, item_type_id) {
        list_type_id -> Uuid,
        item_type_id -> Uuid,
    }
}

diesel::table! {
    friendships (id) {
        id -> Uuid,
        user_who_asked_id -> Uuid,
        user_who_got_asked_id -> Uuid,
        is_validated -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ItemTypeVariation;

    item_types (id) {
        id -> Uuid,
        item_type_variation -> ItemTypeVariation,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PriorityType;

    items (id) {
        id -> Uuid,
        content -> Nullable<Varchar>,
        is_checked -> Nullable<Bool>,
        media_url -> Nullable<Varchar>,
        website_url -> Nullable<Varchar>,
        person_in_charge -> Nullable<Varchar>,
        priority_type -> Nullable<PriorityType>,
        list_id -> Uuid,
        item_type_id -> Uuid,
        deadline_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    list_tags (id) {
        id -> Uuid,
        name -> Varchar,
        list_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    list_types (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    lists (id) {
        id -> Uuid,
        name -> Varchar,
        list_type_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NotifType;

    notifications (id) {
        id -> Uuid,
        has_been_read -> Bool,
        notif_type -> NotifType,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ListPermission;

    user_lists (user_id, list_id) {
        user_id -> Uuid,
        list_id -> Uuid,
        is_owner -> Bool,
        is_validated -> Bool,
        list_permission -> ListPermission,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        keycloak_uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(allowed_items -> item_types (item_type_id));
diesel::joinable!(allowed_items -> list_types (list_type_id));
diesel::joinable!(items -> item_types (item_type_id));
diesel::joinable!(items -> lists (list_id));
diesel::joinable!(list_tags -> lists (list_id));
diesel::joinable!(lists -> list_types (list_type_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(user_lists -> lists (list_id));
diesel::joinable!(user_lists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    allowed_items,
    friendships,
    item_types,
    items,
    list_tags,
    list_types,
    lists,
    notifications,
    user_lists,
    users,
);
