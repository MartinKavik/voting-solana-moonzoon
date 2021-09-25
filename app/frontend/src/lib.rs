use zoon::*;

mod add_party_page;
mod add_voter_page;
mod app;
mod connection;
mod parties_page;
mod router;
mod solana_helpers;
mod theme;

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", app::root);
    router::router();
    connection::connection();
}
