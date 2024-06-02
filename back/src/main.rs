use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use graphql_logic::db::get_pool;
use graphql_logic::endpoints::graphql_endpoints;
use r2d2::Pool;
use web_socket_logic::web_socket::{ws_handler, NotificationServer};

mod graphql_logic;
mod models;
mod schema;
mod services;
mod web_socket_logic;

#[derive(Clone)]
struct AppState {
    conn: Pool<ConnectionManager<PgConnection>>,
}

fn logging_setup() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv::dotenv().ok();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    let pool = get_pool();
    let state = AppState { conn: pool };

    let keycloak_pk = std::env::var("KEYCLOAK_PK").expect("KEYCLOAK_PK not found in .env file");
    let keycloak_pk =
        format!("-----BEGIN PUBLIC KEY-----\n{keycloak_pk}\n-----END PUBLIC KEY-----");

    // Initialize the notification server
    let notification_server = NotificationServer::new().start();

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
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(notification_server.clone()))
            .service(web::scope("api").wrap(keycloak_auth))
            .configure(graphql_endpoints)
            .route("/ws/{user_id}", web::get().to(ws_handler))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
