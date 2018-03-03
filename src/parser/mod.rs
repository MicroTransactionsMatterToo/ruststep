use NoteType;
use std::boxed::Box;
use std::collections::HashMap;

// Represents a parsed stepfile
pub trait Stepfile {
    /// Returns a list of `StepfileNote`'s at the given `beat_num`.
    /// The list returned is a list of `Box`ed references
    fn get_beat(&self, beat_num: u64) -> StepfileBeat;
    fn gb(&self, beat_num: u64) -> StepfileBeat;
    /// Gets the beats in the file from `beat_start` to `beat_end`
    fn get_beats(&self, beat_start: u64, beat_end: u64) -> Vec<StepfileBeat>;
    /// Shorthand for `get_beats`. See documentation for that method instead
    fn gbs(&self, beat_start: u64, beat_end: u64) -> Vec<StepfileBeat>;
    /// Gets beats that are of the specified type. If your stepfile language has a note type that is
    /// not already implemented in the library, add it to `NoteType` for future use.
    fn get_beats_with_type(&self, note_type: NoteType) -> Vec<StepfileBeat>;
    /// Shorthand for `get_beats_with_type`.  See documentation for that method instead
    fn gbs_wt(&self, note_type: NoteType) -> Vec<StepfileBeat>;
    /// Returns a list of `StepfileBeat`'s that **MATCH** the given set of directions. This parsing
    /// is up to the specific parser for each filetype to implement, but where possible,
    /// the selector syntax should match that of the DWI file formats, which should cover most
    /// stepfile types
    fn get_beats_with_direction(&self, direction: String) -> Vec<StepfileBeat>;
    /// Shorthand version of `get_beats_with_direction`. See documentation for that method instead
    fn gbs_wd(&self, direction: String) -> Vec<StepfileBeat>;
}

pub trait StepfileNote {
    fn note_type(&self) -> NoteType;
    fn beat_number(&self) -> u64;
    /// Returns a direction string. This is different for each languages parser, and one should
    /// refer to the documentation for the relevant file type. This is because stepfile
    /// formats can vary greatly in regards to what could be called a "direction".
    /// For the most part, these stepfiles should share the DWI file formats system of describing
    /// directions
    fn direction(&self) -> String;
    fn attribute(&self, key: String) -> String;
    fn attributes(&self) -> &HashMap<String, String>;
}

pub trait StepfileDirectionMapper {

}

pub trait Parser {

}

// Types
pub type StepfileBeat = Vec<Box<StepfileNote>>;