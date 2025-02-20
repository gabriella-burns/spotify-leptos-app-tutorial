use leptos::prelude::*;
use crate::SpotifyTrack;

#[component]
pub fn TopTracksUI() -> impl IntoView {
    view! {
        <div>
            <h1>"Top Tracks"</h1>
            <ul>
                // {tracks.into_iter().map(|track| view! {
                //     <li>{track.name.clone()} - Popularity: {track.popularity}</li>
                // }).collect_view()}
            </ul>
        </div>
    }
}
