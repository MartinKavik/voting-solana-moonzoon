use zoon::*;

mod connection;
mod app;
mod add_party_page;
mod add_voter_page;
mod home_page;
mod theme;
mod router;

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", app::root);
    router::router();
    connection::connection();
}
