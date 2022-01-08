## Day 6

### Efficiency
- **Part One?** Quick and easy! The `Option` type worked great for possibly spawning a new fish. Those accumulated in a reusable `Vec<Fish>` which we sliced, used to extend the master list of fish after each day, and then cleared going into the next.
- **Part Two?** Time to change the model. Bumping to 256 days caused the program to hang a minute plus. Instead, tried counting fish by their ages in a `School` model, a simple wrapper around a `HashMap`.

### HashMap
- Comes with a rich API, lots more bells and whistles than Go maps
- Upsert, filter entries, iterate through keys or values only, and so on!
- Documentation: https://doc.rust-lang.org/std/collections/struct.HashMap.html