use zoon::*;
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub enum Theme {
    White,
    Green,
    Black,
    Transparent,
}

impl Color<'_> for Theme {}

impl<'a> IntoCowStr<'a> for Theme {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Theme::White => hsl(0, 0, 100),
            Theme::Green => hsl(143.1, 98.8, 84.6),
            Theme::Black => hsl(0, 0, 0),
            Theme::Transparent => hsla(0, 0, 0, 0),
        }
        .into_cow_str()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
