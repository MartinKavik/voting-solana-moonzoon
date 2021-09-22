use zoon::*;
use solana_sdk::hash::Hash;

mod view;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    AddVoter,
    AddParty,
    Parties,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

#[static_ref]
pub fn recent_blockhash() -> &'static Mutable<Option<Hash>> {
    Mutable::new(None)
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

pub fn set_recent_blockhash(blockhash: Hash) {
    recent_blockhash().set_neq(Some(blockhash));
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    view::root()
}

