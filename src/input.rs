#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "steralization", derive(Steralize, Desteralize))]

pub enum GameboyButton
{
    A,
    B,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    START,
    SELECT,
}