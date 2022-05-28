use bracket_lib::prelude::FontCharType;
pub use create::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render{
    pub color: ColorPair;
    pub gylph: FontCharType;
}

pub struct Player;