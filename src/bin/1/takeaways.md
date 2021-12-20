## Day 1

### Iterators
- Any collection can become an `Iterator` if it implements the `next` method
- `Iterator` has many other methods, but all of them use `next`; and so only it needs to be custom implemented
- `next` is already implemented for standard collections like vectors, arrays, and slices
- For vectors, `for x in xs` calls `into_iter()` under the hood while `for x in &xs` calls `iter()`
- For slices, `for x in xs` calls `iter()` under the hood
- `next` requires a mutable reference to the `Iterator` itself
- Several helper methods convert a collection into an `Iterator`:
    - `iter` for read-only iteration by reference (borrow)
    - `iter_mut` for mutable iteration by reference (borrow)
    - `into_iter` for read-only iteration that consumes the underlying collection (move)
- Documentation: https://doc.rust-lang.org/std/iter/index.html

### Match
- Can be used as an expression itself to assign value to a variable – if so, followed by a `;`
- Can also be used to control flow between several case-based expressions
- Examples: https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html