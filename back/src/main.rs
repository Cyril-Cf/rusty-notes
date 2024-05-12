use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod services;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

const KEYCLOAK_PK: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxRwn7H5aSeOrCW7KQwx+gzuqCsAnrxQf0kO46ngGLuAG2G1YCkY+h3h/0rEH8pJ9Doo9sFDmFuiW0m23wcI6ZJGklMwRjatdepB2PcPKTP1UKR0LDdQtQu/SkPYoNypj2Irj12MAcPNSKIRNIYQ0mmhqnp09EQ6K1mlff7Tzwtk1dTnGZ3qc370WEPZCDYgsXIAxWyCFMcfHnG50cAF/QvX52CpBqfcaaf9o/CGNXgp527i5gCRIjm9zyaSGDdE8r2tQE2scHfT9rm2+7wAIAhOy+2fU8+zcNUAu1HAigxedYFbnp6spwlwW053Xv0EtMj1OXig1Qkj+PdW4YiqzIQIDAQAB
-----END PUBLIC KEY-----";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let conn = Database::connect(db_url).await.unwrap();
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
    println!("Backend launched!");
    let state = AppState { conn };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let keycloak_auth = KeycloakAuth {
            detailed_responses: true,
            passthrough_policy: AlwaysReturnPolicy,
            keycloak_oid_public_key: DecodingKey::from_rsa_pem(KEYCLOAK_PK.as_bytes()).unwrap(),
            required_roles: vec![],
        };
        App::new()
            .wrap(cors)
            .service(
                web::scope("api")
                    .wrap(keycloak_auth)
                    .service(controllers::user_controller::add_user)
                    .service(controllers::user_controller::find_one_user)
                    .service(controllers::user_controller::find_all_users)
                    .service(controllers::user_controller::update_user)
                    .service(controllers::user_controller::delete_user)
                    .service(controllers::permission_controller::add_permission)
                    .service(controllers::permission_controller::find_all_permissions)
                    .service(controllers::permission_controller::find_one_permission)
                    .service(controllers::permission_controller::delete_permission)
                    .service(controllers::user_permission_controller::add_permission_to_user)
                    .service(controllers::user_permission_controller::get_permissions_for_user)
                    .service(controllers::user_permission_controller::remove_permissions_for_user),
            )
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
