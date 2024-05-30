// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_type"))]
    pub struct ItemType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_type"))]
    pub struct ListType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "notification_type"))]
    pub struct NotificationType;
}

diesel::table! {
    friendships (id) {
        id -> Uuid,
        user_id -> Uuid,
        user_id2 -> Uuid,
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
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NotificationType;

    notifications (id) {
        id -> Uuid,
        content -> Varchar,
        has_been_read -> Bool,
        notification_type -> NotificationType,
        user_id -> Uuid,
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
diesel::joinable!(lists -> users (user_id));
diesel::joinable!(notifications -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    friendships,
    items,
    list_tags,
    lists,
    notifications,
    users,
);
