use leptos::prelude::*;
use leptos::*;
use leptos_router::path;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SpotifyTrack {
    name: String,
    popularity: u32,
}

#[cfg(feature = "ssr")]
mod ssr_imports {
    use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
    use dotenvy;
    use leptos_actix::*;
    use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AuthResponse {
    access_token: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopTracksResponse {
    tracks: Vec<SpotifyTrack>,
}

// This struct represents state
#[cfg(feature = "ssr")]
struct AppState {
    app_name: String,
    client_id: String,
    client_secret: String,
}

#[cfg(feature = "ssr")]
#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[cfg(feature = "ssr")]
async fn get_access_token(client_id: &str, client_secret: &str) -> Result<String, Box<dyn Error>> {


    let client = reqwest::Client::new();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await?;

    let auth_response: AuthResponse = response.json().await?;
    Ok(auth_response.access_token)
}

#[cfg(feature = "ssr")]
async fn get_artist_top_tracks(
    access_token: &str,
    artist_id: &str,
) -> Result<TopTracksResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.spotify.com/v1/artists/{}/top-tracks?market=US",
        artist_id
    );

    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let tracks: TopTracksResponse = response.json().await?;
    Ok(tracks)
}

#[cfg(feature = "ssr")]
#[get("/top-tracks/{artist_id}")]
async fn top_tracks(data: web::Data<AppState>) -> impl Responder {
    let artist_id = "4Z8W4fKeB5YxbusRsdQVPb";

    // let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    // let client_secret =
    //     env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");

    let access_token = match get_access_token(&data.client_id, &data.client_secret).await {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get access token: {}", e))
        }
    };

    match get_artist_top_tracks(&access_token, &artist_id).await {
        Ok(top_tracks) => HttpResponse::Ok().json(top_tracks),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to get top tracks: {}", e))
        }
    }
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");

    _ = console_log::init_with_level(log::Level::Debug);
    // why move - having a move in front of the closure is necessary because we are capturing the client_id and client_secret variables from the outer scope.
    // this allows it to exist as the app runs
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Spotify App"),
                client_id: client_id.clone(),
                client_secret: client_secret.clone(),
            }))
            .service(top_tracks)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
fn main() {
    use pls::App;
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
