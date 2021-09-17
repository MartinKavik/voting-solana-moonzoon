use zoon::*;

mod view;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    AddVoter,
    AddParty,
    Home,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//    Signals
// ------ ------

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    view::root()
}

