pub use crate::prelude::*;
use bracket_lib::prelude::FontCharType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub gylph: FontCharType,
}

pub struct Player;
