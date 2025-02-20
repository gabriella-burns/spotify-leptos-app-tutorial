use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
mod tracks;
use serde::{Deserialize, Serialize};
use tracks::TopTracksUI;

/// struct which defines the spotify track
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SpotifyTrack {
    name: String,
    popularity: u32,
}


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos spotify app."/>
        <Router set_is_routing>
            // shows a progress bar while async data are loading
            <main>
                <FlatRoutes fallback=|| "Not found.">
                    <Route path=StaticSegment("") view=TopTracksUI/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
