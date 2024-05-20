use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list_tag_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_list: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_tag_list: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::IdList",
        to = "super::list::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
    #[sea_orm(
        belongs_to = "super::list_tag::Entity",
        from = "Column::IdTagList",
        to = "super::list_tag::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Listtag,
}

impl Related<super::list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl Related<super::list_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}