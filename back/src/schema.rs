// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_type"))]
    pub struct ItemType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_permission"))]
    pub struct ListPermission;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_type"))]
    pub struct ListType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "notif_type"))]
    pub struct NotifType;
}

diesel::table! {
    friendships (id) {
        id -> Uuid,
        user_who_asked_id -> Uuid,
        user_who_got_asked_id -> Uuid,
        is_validated -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ItemType;

    items (id) {
        id -> Uuid,
        name -> Varchar,
        is_checked -> Bool,
        list_id -> Uuid,
        item_type -> ItemType,
    }
}

diesel::table! {
    list_tags (id) {
        id -> Uuid,
        name -> Varchar,
        list_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ListType;

    lists (id) {
        id -> Uuid,
        name -> Varchar,
        list_type -> ListType,
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
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        keycloak_uuid -> Uuid,
    }
}

diesel::joinable!(items -> lists (list_id));
diesel::joinable!(list_tags -> lists (list_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(user_lists -> lists (list_id));
diesel::joinable!(user_lists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    friendships,
    items,
    list_tags,
    lists,
    notifications,
    user_lists,
    users,
);
