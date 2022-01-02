## Day 3

### Into
- This trait has many built-in implementations for core types, so we can quietly cast – for instance – a `Result<usize, String>` into `Result<u16, String>` when returning up one function from another including with the `?` operator.
- Documentation: https://doc.rust-lang.org/std/convert/trait.Into.html

### Standard Library
- Is awesome. Some favorite finds were the `retain` method for filtering vectors and `u16`'s `str_from_radix` utility function to cast string representation of numbers with any base – including binary for today's exercise – back into a number.

## Enum Type
- Was a very convenient way to simplify the function for applying "bit criteria." Great tool for representing exhaustive cases concisely.