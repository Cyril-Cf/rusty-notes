use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};

#[get("/users")]
async fn users() -> impl Responder {
    HttpResponse::Ok().body("hello from the API!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

const KEYCLOAK_PK: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxRwn7H5aSeOrCW7KQwx+gzuqCsAnrxQf0kO46ngGLuAG2G1YCkY+h3h/0rEH8pJ9Doo9sFDmFuiW0m23wcI6ZJGklMwRjatdepB2PcPKTP1UKR0LDdQtQu/SkPYoNypj2Irj12MAcPNSKIRNIYQ0mmhqnp09EQ6K1mlff7Tzwtk1dTnGZ3qc370WEPZCDYgsXIAxWyCFMcfHnG50cAF/QvX52CpBqfcaaf9o/CGNXgp527i5gCRIjm9zyaSGDdE8r2tQE2scHfT9rm2+7wAIAhOy+2fU8+zcNUAu1HAigxedYFbnp6spwlwW053Xv0EtMj1OXig1Qkj+PdW4YiqzIQIDAQAB
-----END PUBLIC KEY-----";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
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
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").wrap(keycloak_auth).service(users))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
