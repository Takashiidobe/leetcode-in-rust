## Regex

Rust, unlike many other languages, does not have regex in its standard
library. This is because of its tight integration to crates: if regex is
required, it's assumed that you can download the `regex` crate from
crates.io.

More documentation can be found here: [Regex Docs](https://docs.rs/regex/latest/regex/).

Some common use cases include:

### Matching on
