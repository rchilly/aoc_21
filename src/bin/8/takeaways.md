## Day 8

### String Contains
- Beware! The `contains` method comments say it accepts a `&[char]`, but this did not work as expected. I had to write my own custom function to verify that one string contained ALL the characters in another.
- Passed a `&[char]`, the method returns true if the string contains _any_ of those characters. Which is nowhere documented in the actual method comments. Rather, in the comments for the `Pattern` type it accepts: https://doc.rust-lang.org/stable/src/core/str/pattern.rs.html#57-68