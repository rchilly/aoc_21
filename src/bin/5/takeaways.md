## Day 5

### Const Generics
- Turned out to be the wrong tool for the job with my `Grid` type.
- Originally implemented with const generics for width and height, but with a 990x990 grid for this puzzle – ~1M points – we overflowed the stack! Oops.
- Using `Vec` for the rows solved the problem – leaning on the heap instead – and simplified code too.