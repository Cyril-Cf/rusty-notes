use crate::models::list::List;
use crate::models::user::User;
use crate::schema::user_lists;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::GraphQLEnum;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(table_name = user_lists)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(List))]
#[diesel(primary_key(user_id, list_id))]
pub struct UserList {
    pub user_id: Uuid,
    pub list_id: Uuid,
    pub is_owner: bool,
    pub is_validated: bool,
    pub list_permission: ListPermission,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, Clone, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ListPermission"]
pub enum ListPermission {
    #[db_rename = "OWNER"]
    Owner,
    #[db_rename = "CAN_SEE_BUT_NOT_MODIFY"]
    CanSeeButNotModify,
    #[db_rename = "CAN_SEE_AND_MODIFY"]
    CanSeeAndModify,
}

#[derive(Insertable)]
#[diesel(table_name = user_lists)]
pub struct NewUserList {
    pub user_id: Uuid,
    pub list_id: Uuid,
    pub is_owner: bool,
    pub is_validated: bool,
    pub list_permission: ListPermission,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddUserToListStatus {
    AddSuccessful,
    ErrCannotAdd,
}

#[derive(Debug, GraphQLEnum)]
pub enum RemoveUserFromListStatus {
    RemoveSuccessful,
    ErrCannotRemove,
}
