use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use graphql::{create_schema, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

mod controllers;
mod graphql;
mod services;

#[post("/graphql")]
async fn graphql_entrypoint(
    app_data: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
) -> impl Responder {
    let ctx = graphql::Context {
        db: app_data.conn.clone(),
    };

    let res = data.execute(&schema, &ctx).await;
    serde_json::to_string(&res).unwrap()
}

#[get("/graphiql")]
async fn graphiql() -> impl Responder {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok().body(html)
}

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let keycloak_pk = std::env::var("KEYCLOAK_PK").expect("KEYCLOAK_PK not found in .env file");
    let keycloak_pk =
        format!("-----BEGIN PUBLIC KEY-----\n{keycloak_pk}\n-----END PUBLIC KEY-----");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let conn = Database::connect(&db_url).await.unwrap();
    println!("Emptying DB...");
    Migrator::down(&conn, None).await.unwrap();
    println!("DB empty!");
    println!("Checking migrations...");
    let pending_migrations = Migrator::get_pending_migrations(&conn).await.unwrap();
    if pending_migrations.is_empty() {
        println!("No migration pending.")
    } else {
        for migration in pending_migrations {
            println!("migration pending: {}", migration.name());
        }
        Migrator::up(&conn, None).await.unwrap();
        println!("Migrations applied!");
    }
    let schema = Arc::new(create_schema());
    let state = AppState { conn };

    println!("Backend launched!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let keycloak_auth = KeycloakAuth {
            detailed_responses: true,
            passthrough_policy: AlwaysReturnPolicy,
            keycloak_oid_public_key: DecodingKey::from_rsa_pem(keycloak_pk.as_bytes()).unwrap(),
            required_roles: vec![],
        };
        App::new()
            .wrap(cors)
            .app_data(web::Data::from(schema.clone()))
            .app_data(web::Data::new(state.clone()))
            .service(graphql_entrypoint)
            .service(graphiql)
            .service(web::scope("api").wrap(keycloak_auth))
            .service(controllers::test::test)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
