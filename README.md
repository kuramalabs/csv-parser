# csv-parser

A tiny zero-copy CSV parser built from scratch in Rust - no dependencies, just iterators.

`CsvReader` walks through lines, `CsvRow` walks through fields. Both implement `Iterator`, so you get the usual Rust ergonomics for free.

```rust
let input = "name,age\nAlice,30";
let mut reader = CsvReader::new(input);

for field in reader.next().unwrap() {
    println!("{field}"); // name, age
}
```
