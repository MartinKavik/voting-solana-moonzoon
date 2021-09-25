#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::semicolon_if_nothing_returned,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::wildcard_imports,
    clippy::future_not_send
)]

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
