# univar

Mock client of [univrs](https://github.com/reece/univrs). Hello world in both languages:

```
cargo run          # Rust: build model, serialize with serde_json
uvx univar         # Python: build model, serialize in Rust, print the Python str
```

Both print:

```
{"location":{"end":101,"sequence_id":"NC_000001.11","start":100},"state":"T"}
```
