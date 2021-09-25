use crate::theme::Theme;
use zoon::*;

pub fn page() -> impl Element {
    Column::new()
        .s(Height::fill().max(500))
        .s(Padding::new().y(10))
        .item(
            Column::new()
                .s(Width::fill().max(550))
                .s(Background::new().color(Theme::Transparent))
                .s(Align::center())
                .s(Font::new().color(Theme::White))
                .s(Spacing::new(10))
                .s(Padding::all(30))
                .s(Spacing::new(25))
                .s(RoundedCorners::all(25))
                .item(title())
                .item(add_party_fields())
                .item(status())
                .item(add_voter_button()),
        )
}

fn title() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .s(Font::new().size(40).weight(NamedWeight::SemiBold))
        .child("Add Party")
}

fn add_party_fields() -> impl Element {
    Column::new()
        .s(Spacing::new(25))
        .item(fee_payer_private_key_field())
        .item(party_name_field())
}

fn fee_payer_private_key_field() -> impl Element {
    Column::new()
        .s(Spacing::new(8))
        .item(fee_payer_private_key_label())
        .item(fee_payer_private_key_input())
}

fn fee_payer_private_key_label() -> impl Element {
    Label::new()
        .for_input("fee_payer_private_key")
        .label("Fee payer private key")
}

fn fee_payer_private_key_input() -> impl Element {
    TextInput::new()
        .id("fee_payer_private_key")
        .s(Padding::all(6))
        .on_change(super::set_fee_payer_private_key)
        .text_signal(super::fee_payer_private_key().signal_cloned())
        .on_key_down(|event| event.if_key(Key::Enter, super::add_party))
        .placeholder(Placeholder::new("[XX, XXX, ...]"))
}

fn party_name_field() -> impl Element {
    Column::new()
        .s(Spacing::new(8))
        .item(party_name_label())
        .item(party_name_input())
}

fn party_name_label() -> impl Element {
    Label::new().for_input("party_name").label("Party name")
}

fn party_name_input() -> impl Element {
    TextInput::new()
        .id("party_name")
        .s(Padding::all(6))
        .on_change(super::set_party_name)
        .text_signal(super::party_name().signal_cloned())
        .on_key_down(|event| event.if_key(Key::Enter, super::add_party))
        .placeholder(Placeholder::new("New Party"))
}

fn status() -> impl Element {
    El::new().child_signal(super::status().signal_cloned())
}

fn add_voter_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| Theme::White, || Theme::Green)))
        .s(Font::new()
            .color(Theme::Black)
            .weight(NamedWeight::Bold)
            .size(16))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::add_party)
        .label("ADD PARTY")
}
