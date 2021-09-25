use crate::{router::Route, theme::Theme};
use zoon::*;

pub fn root() -> impl Element {
    Column::new()
        .s(Height::fill().min_screen())
        .item(header())
        .item_signal(super::page_id().signal().map(page))
}

fn header() -> impl Element {
    Row::with_tag(Tag::Nav)
        .s(Height::new(64))
        .s(Font::new().color(Theme::White))
        .s(Align::new().center_x())
        .s(Spacing::new(5))
        .items(header_links())
}

fn header_links() -> Vec<impl Element> {
    vec![
        header_link(Route::Root, "Parties", super::PageId::Parties),
        header_link(Route::AddParty, "Add Party", super::PageId::AddParty),
        header_link(Route::AddVoter, "Add Voter", super::PageId::AddVoter),
    ]
}

fn header_link(route: Route, label: &str, page_id: super::PageId) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_or_selected = map_ref! {
        let hovered = hovered_signal,
        let current_page_id = super::page_id().signal() => move {
            *hovered || *current_page_id == page_id
        }
    };
    Link::new()
        .s(Height::fill())
        .s(Padding::new().x(12))
        .s(Borders::new().bottom_signal(hovered_or_selected.map_bool(
            || Border::new().color(Theme::Green).width(5),
            || Border::new().color(Theme::Transparent).width(5),
        )))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(route)
        .label(Row::new().s(Height::fill()).item(label))
}

fn page(page_id: super::PageId) -> impl Element {
    match page_id {
        super::PageId::AddVoter => crate::add_voter_page::view(),
        super::PageId::AddParty => crate::add_party_page::view(),
        super::PageId::Parties => crate::parties_page::view(),
        super::PageId::Unknown => unknown_page().into_raw_element(),
    }
}

fn unknown_page() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Font::new().color(Theme::White).size(50))
        .child(404)
}
