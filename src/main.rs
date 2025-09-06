extern crate rocket;
use gemini::GeminiConfig;
use tokio;
use rocket::fs::FileServer;
use rocket::response::Debug;
use rocket::{catch, catchers, launch, routes, uri, Request};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::http::{Cookie, SameSite};
use rocket_oauth2::OAuth2;
//use crate::auth::rocket_uri_macro_google_login;
//use crate::claims::{AppState, Google, JwtConfig};
use crate::claims::{AppState};
use reqwest::Client;

pub mod db;
pub mod cotizaciones;
pub mod claims;
pub mod gemini;
pub mod personas;

pub static STATIC_FILES_DIR: &str = "www/static";

/*
#[tokio::main]
async fn main() {
    println!("PoC CV creation:");
    let output = generate("
        Nombre: Iván Chavero
        Posición: Principal Software Engineer
        Habilidades: AI, MLOps, Linux, Rust, Unix, Shell Scripting, Virtualization, Programming, Computer science, Distributed systems,
                     Android Development, Full Stack, Python, Perl, PHP, Red Hat, Debian, Ruby, Java, CSS, HTML5, JQuery, Javascript,
                     OCI Containers, docker, podman, LDAP, problem solving, critical thinking, Go language, Kubernetes, pascal, CentOS,
                     Destkop application development, Open Source, Free Software
        Posiciones anteriores: Red Hat: Principal software engineer
    ".into()).await.unwrap();

    let title = "Iván Chavero CV";
    let formatted_title = format!("{:-^1$}", title, 50);
    println!("\n{}\n\n{}\n\n{}\n", formatted_title, output, formatted_title);
}
*/

#[launch]
fn rocket() -> _ {
   // Load Rocket's configuration (this will look for Rocket.toml)
    let figment = rocket::Config::figment();

    /* TODO: enable when authentication is activated
    // Extract JWT configuration from the "jwt" table in Rocket.toml
    let jwt_config: JwtConfig = figment
        .extract_inner("jwt")
        .expect("JWT configuration missing in Rocket.toml");

    */

     let gemini_config: GeminiConfig = figment
            .extract_inner("gemini")
            .expect("Gemini API key missing from Rocket.toml");

    let app_state = AppState {
        http_client: Client::new(),
        gemini_api_key: gemini_config.api_key,
        //jwt_secret: jwt_config.secret,
    };

    rocket::build()
        /* for google auth .mount(
            "/",
            routes![
                auth::logout,
                auth::google_login,
                auth::google_callback,
                auth::logged,
        ])*/
        /*.mount(
            "/users",
            routes![users::me,
        ])*/
        .mount(
            "/cotizaciones",
            routes![
                cotizaciones::index,
                cotizaciones::generate,
        ])
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .manage(app_state)
        //TODO .register("/", catchers![unauthorized_catcher])
        .attach(Template::fairing())
        .attach(db::PajaDB::fairing())
        //.attach(OAuth2::<Facebook>::fairing("facebook"))

}

/* TODO
// Catcher that handles auth failures and redirects
#[catch(403)]
fn unauthorized_catcher(req: &Request) -> Redirect {
    //TODO: Try to get the cached redirect
    //req.local_cache(|| Redirect::to(uri!(facebook_login)))
    // TODO: check the use of cache
    let uri = req.uri().to_string();
    let cookies = req.cookies();
    let cookie = Cookie::build(("login_uri", uri.clone()))
        .path("/")
        .secure(false)
        .same_site(SameSite::Lax)
        .http_only(false);
    cookies.add(cookie);
    Redirect::to(uri!(facebook_login))
}*/
