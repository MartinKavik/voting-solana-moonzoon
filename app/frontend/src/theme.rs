use std::borrow::Cow;
use zoon::*;

#[derive(Clone, Copy)]
pub enum Theme {
    White,
    Green,
    Red,
    Black,
    Gray,
    Transparent,
}

impl Color<'_> for Theme {}

impl<'a> IntoCowStr<'a> for Theme {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Theme::White => hsl(0, 0, 100),
            Theme::Green => hsl(143.1, 98.8, 84.6),
            Theme::Red => hsl(18.7, 100, 61.9),
            Theme::Black => hsl(0, 0, 0),
            Theme::Gray => hsl(0, 0, 50),
            Theme::Transparent => hsla(0, 0, 0, 0),
        }
        .into_cow_str()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
