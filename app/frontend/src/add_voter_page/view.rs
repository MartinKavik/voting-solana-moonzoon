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
                .item(add_voter_fields())
                .item(status())
                .item(add_voter_button()),
        )
}

fn title() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .s(Font::new().size(40).weight(NamedWeight::SemiBold))
        .child("Add Voter")
}

fn add_voter_fields() -> impl Element {
    Column::new()
        .s(Spacing::new(25))
        .item(voting_owner_private_key_field())
        .item(voter_pubkey_field())
}

fn voting_owner_private_key_field() -> impl Element {
    Column::new()
        .s(Spacing::new(8))
        .item(voting_owner_private_key_label())
        .item(voting_owner_private_key_input())
}

fn voting_owner_private_key_label() -> impl Element {
    Label::new()
        .for_input("voting_owner_private_key")
        .label("Voting Owner private key")
}

fn voting_owner_private_key_input() -> impl Element {
    TextInput::new()
        .id("voting_owner_private_key")
        .s(Padding::all(6))
        .on_change(super::set_voting_owner_private_key)
        .text_signal(super::voting_owner_private_key().signal_cloned())
        .on_key_down(|event| event.if_key(Key::Enter, super::add_voter))
        .placeholder(Placeholder::new("[XX, XXX, ...]"))
}

fn voter_pubkey_field() -> impl Element {
    Column::new()
        .s(Spacing::new(8))
        .item(voter_pubkey_label())
        .item(voter_pubkey_input())
}

fn voter_pubkey_label() -> impl Element {
    Label::new()
        .for_input("voter_pubkey")
        .label("Voter public key")
}

fn voter_pubkey_input() -> impl Element {
    TextInput::new()
        .id("voter_pubkey")
        .s(Padding::all(6))
        .on_change(super::set_voter_pubkey)
        .text_signal(super::voter_pubkey().signal_cloned())
        .on_key_down(|event| event.if_key(Key::Enter, super::add_voter))
        .placeholder(Placeholder::new("PubKey"))
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
        .on_press(super::add_voter)
        .label("ADD VOTER")
}
