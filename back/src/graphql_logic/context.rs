use super::db::PostgresPool;
use crate::NotificationServer;
use actix::Addr;

pub struct GraphQLContext {
    pub pool: PostgresPool,
    pub notification_server: Addr<NotificationServer>,
}

impl juniper::Context for GraphQLContext {}
