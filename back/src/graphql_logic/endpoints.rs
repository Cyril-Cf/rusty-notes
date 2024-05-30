use super::context::GraphQLContext;
use super::graphql::create_schema;
use super::graphql::Schema;
use crate::AppState;
use crate::NotificationServer;
use actix::Addr;
use actix_web::{web, HttpResponse, Responder};
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(web::Data::new(schema))
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}

async fn graphql(
    state: web::Data<AppState>,
    schema: web::Data<Arc<Schema>>,
    notification_server_data: web::Data<Addr<NotificationServer>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let pool = state.conn.clone();
    let notification_server = notification_server_data.get_ref().clone();
    let ctx = GraphQLContext {
        pool,
        notification_server: notification_server.clone(),
    };
    let res = data.execute(&schema, &ctx).await;
    serde_json::to_string(&res).unwrap()
}
