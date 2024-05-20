use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

// mod controllers;
// mod services;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

const KEYCLOAK_PK: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmdwo76aI5UQEBYQKlW6d7PmX95+RcK+LT7a6htP84vNVsAw39nj5BoR8MNd2YTfFxh+3W/UZNJPy33GPMtHFvDf0dpwJr1X5PIv+1PL7dk3DL1O+rQVlmFDRPGQVZ4NkEXgnAcTztYvcQxm/jv3dkD3D9bt2BYFEuOnPW0zYLFtlRZX5sZuRwP69CK3csURduM4gGYDs65uyTb6xL8izcGvVsOCNGQ12pgrCziQTN9dwA1BSJciajD+H1hYVvKeyLDuFE8or5+ibePvC7v8RpPQR1j7TH43jGAKjkEmEJncZxd8cAV3geoSasquZB8up3YZ5Web1OY7dBVo0CJlh3QIDAQAB
-----END PUBLIC KEY-----";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let conn = Database::connect(db_url).await.unwrap();
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
                    // .service(controllers::user_controller::add_user)
                    // .service(controllers::user_controller::find_one_user)
                    // .service(controllers::user_controller::find_all_users)
                    // .service(controllers::user_controller::update_user)
                    // .service(controllers::user_controller::delete_user)
                    // .service(controllers::permission_controller::add_permission)
                    // .service(controllers::permission_controller::find_all_permissions)
                    // .service(controllers::permission_controller::find_one_permission)
                    // .service(controllers::permission_controller::delete_permission)
                    // .service(controllers::user_permission_controller::add_permission_to_user)
                    // .service(controllers::user_permission_controller::get_permissions_for_user)
                    // .service(controllers::user_permission_controller::remove_permissions_for_user),
            )
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
