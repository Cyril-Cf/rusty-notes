use crate::models::list::List;
use crate::models::user::User;
use crate::schema::user_lists;
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
}

#[derive(Insertable)]
#[diesel(table_name = user_lists)]
pub struct NewUserList {
    pub user_id: Uuid,
    pub list_id: Uuid,
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
