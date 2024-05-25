use juniper::GraphQLObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, GraphQLObject)]
#[sea_orm(table_name = "users")]
#[graphql(description = "A user in the system")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub keycloak_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_list::Entity")]
    Userlist,
    #[sea_orm(has_many = "super::user_notification::Entity")]
    Usernotification,
    #[sea_orm(has_many = "super::list_tags::Entity")]
    Listtag,
    #[sea_orm(has_many = "super::friendships::Entity")]
    Friendship,
}

impl Related<super::user_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Userlist.def()
    }
}

impl Related<super::user_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usernotification.def()
    }
}

impl Related<super::list_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtag.def()
    }
}

impl Related<super::friendships::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Friendship.def()
    }
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_list::Relation::List.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_list::Relation::User.def().rev())
    }
}

impl Related<super::notifications::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_notification::Relation::Notification.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_notification::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "customer"
    }
}
