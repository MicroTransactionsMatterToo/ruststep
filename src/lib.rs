// Module Definitions
pub mod parser;


// Global Struct Defs

/// Note Types, corresponding to respective divisions of bars. A fourth note in a `4/4` piece is one beat of a bar
pub enum NoteType {
    Fourth,
    Eighth,
    Twelfth,
    Sixteenth,
    TwentyFourth,
    ThirtySecond,
    FortyEighth,
    SixtyFourth,
    OneNinetySecond
}

