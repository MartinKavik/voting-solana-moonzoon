use crate::{app::PageId, *};

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::AddVoter) => {
            app::set_page_id(PageId::AddVoter);
        }
        Some(Route::AddParty) => {
            app::set_page_id(PageId::AddParty);
        }
        Some(Route::Root) => {
            parties_page::request_deadline();
            parties_page::request_parties();
            app::set_page_id(PageId::Parties);
        }
        None => {
            app::set_page_id(PageId::Unknown);
        }
    })
}

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
pub enum Route {
    #[route("add_voter")]
    AddVoter,

    #[route("add_party")]
    AddParty,

    #[route()]
    Root,
}
