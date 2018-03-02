### Common Attributes
- All have metadata
- All have beat divisions that are multiples of 4


### Concepts
Represent all notes as a single type or trait, with the
relevant attributes, like
- Beat Division
- Note Direction (This is language specific, so perhaps a trait then?)
- Note Attributes (hashmap of some sort?, helper function to simplify access?)

### Note Attributes
Primarily used for displaying them, but also valuable information,
these should probably have some sort of parser specific function that
returns a useful string version of it. Otherwise up to users
to use it correctly.

### Holds and Attacks
As these aren't in all languages, but very common
in those that do, they should have an easy to use standard
interface.

### Filtering
All parsers returns should implement various functions
to filter the parsed files content. These should consist of at least.
Information on the beat the notes returned occured should be
included
- Beat Division: `function(enum BEAT_DIVISION) or function(int BEAT_DIVSION)`
- Note Direction: `function(enum ARROW_DIRECTION)`<br>The enum used here
would be language specific, as some languages have different
arrow directions
- Note Attribute: `function(str attribute_key, str attribute_value)`

### Parser Return values
All parsers should return some sort of structure adhering to a trait
that defines the common behaviour between them.

### Returns of Beats
Beats should be an array of `Note` adherent structures, of
varying length.

