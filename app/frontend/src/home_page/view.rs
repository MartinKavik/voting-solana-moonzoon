use zoon::*;
use crate::theme::Theme;

pub fn page() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Font::new().color(Theme::White).size(50))
        .child("parties")
}
