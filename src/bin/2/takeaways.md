## Day 1

### FromString
- Implementing this trait, from the `std::str` module, lets us use the `parse` turbofish method to convert some string slice into our own custom type
- Documentation: https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html

### Results
-The `?` operator short circuit returns from a function if a returned `std::result::Result` contains an `Err` value
- The `Result` type has many useful methods including `.map_err`, which I used to make sure several uses of `?` in a single function would all return results with the same `Err` type
- Documentation: https://doc.rust-lang.org/std/result/

## Custom Errors
- Just have to implement `Display` and `Debug`, with other optional methods available
- Documentation
    - https://learning-rust.github.io/docs/e7.custom_error_types.html
    - https://doc.rust-lang.org/std/error/trait.Error.html