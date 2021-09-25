use crate::theme::Theme;
use solana_sdk::pubkey::Pubkey;
use std::{cmp::Ordering, sync::Arc};
use zoon::*;

pub fn page() -> impl Element {
    Column::new()
        .s(Width::fill().max(550))
        .s(Background::new().color(Theme::Transparent))
        .s(Align::new().center_x())
        .s(Font::new().color(Theme::White))
        .s(Spacing::new(10))
        .s(Padding::new().x(30).y(40))
        .s(Spacing::new(25))
        .s(RoundedCorners::all(25))
        .item(title())
        .item(voter_private_key_field())
        .item_signal(super::deadline().signal().map_some(deadline))
        .item(status())
        .item(parties())
}

fn title() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .s(Font::new().size(40).weight(NamedWeight::SemiBold))
        .child("Parties")
}

// ------ voter_private_key ------

fn voter_private_key_field() -> impl Element {
    Column::new()
        .s(Spacing::new(8))
        .item(voter_private_key_label())
        .item(voter_private_key_input())
}

fn voter_private_key_label() -> impl Element {
    Label::new()
        .for_input("voter_private_key")
        .label("Voter private key")
}

fn voter_private_key_input() -> impl Element {
    TextInput::new()
        .id("voter_private_key")
        .s(Padding::all(6))
        .on_change(super::set_voter_private_key)
        .text_signal(super::voter_private_key().signal_cloned())
        .placeholder(Placeholder::new("[XX, XXX, ...]"))
}

fn deadline(timestamp: i64) -> impl Element {
    Column::new()
        .s(Align::new().center_x())
        .s(Spacing::new(4))
        .item(deadline_date_time(timestamp))
        .item(deadline_countdown(timestamp))
}

// ------ deadline ------

fn deadline_date_time(timestamp: i64) -> impl Element {
    let date_time = Local.timestamp(timestamp, 0);
    El::new().child(date_time.to_rfc2822())
}

fn deadline_countdown(timestamp: i64) -> impl Element {
    let mutable_countdown = Mutable::new(0);
    let countdown = mutable_countdown.read_only();
    let countdown_updater = Timer::new_immediate(1_000, move || {
        mutable_countdown.set_neq(timestamp - Local::now().timestamp());
    });
    Row::new()
        .s(Spacing::new(4))
        .s(Align::new().center_x())
        .s(
            Font::new().color_signal(countdown.signal().map(|countdown| {
                if countdown < 0 {
                    Theme::Red
                } else {
                    Theme::Green
                }
            })),
        )
        .after_remove(move |_| drop(countdown_updater))
        .item(Text::with_signal(countdown.signal()))
        .item(El::new().child("s"))
}

// ------ status ------

fn status() -> impl Element {
    El::new().child_signal(super::status().signal_cloned())
}

// ------ parties ------

fn parties() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Spacing::new(50))
        .items_signal_vec(super::parties().signal_vec_cloned().map(party))
}

fn party(party: Arc<super::Party>) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Spacing::new(15))
        .item(vote_button(party.clone(), false))
        .item(party_data(&party))
        .item(vote_button(party, true))
}

fn vote_button(party: Arc<super::Party>, positive: bool) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(RoundedCorners::all_max())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::White,
            move || if positive { Theme::Green } else { Theme::Red },
        )))
        .s(Font::new()
            .color(Theme::Black)
            .weight(NamedWeight::SemiBold)
            .size(25))
        .s(Width::new(40))
        .s(Height::new(40))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || super::vote(party.pubkey, positive))
        .label(
            El::new()
                .s(Align::center())
                .child(if positive { "+" } else { "-" }),
        )
}

fn party_data(party: &super::Party) -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Spacing::new(8))
        .item(party_name_and_votes(party))
        .item(party_pubkey(party.pubkey))
}

fn party_name_and_votes(party: &super::Party) -> impl Element {
    Row::new()
        .s(Spacing::new(8))
        .item(party_name(&party.name))
        .item(party_votes(&party.votes))
}

fn party_name(name: &str) -> impl Element {
    El::new().s(Font::new().size(20)).child(name)
}

fn party_votes(votes: &Mutable<i64>) -> impl Element {
    El::new()
        .s(Font::new()
            .size(20)
            .color_signal(votes.signal().map(|votes| match votes.cmp(&0) {
                Ordering::Less => Theme::Red,
                Ordering::Equal => Theme::White,
                Ordering::Greater => Theme::Green,
            })))
        .s(Align::new().right())
        .child(Text::with_signal(votes.signal()))
}

fn party_pubkey(pubkey: Pubkey) -> impl Element {
    TextInput::new()
        .s(Width::fill())
        .s(Background::new().color(Theme::Gray))
        .s(Padding::all(3))
        .label_hidden("party public key")
        .read_only(true)
        .text(&pubkey.to_string())
}
