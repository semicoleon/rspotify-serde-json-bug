Demonstrating an issue when deserializing a PlaylistId from a reader with serde_json

Run `cargo test` to see the failure. The `slice` test succeeds but the `reader` test fails despite using the same data.

I *think* the offending line is `let field = <&str>::deserialize(deserializer)?;` at `idtypes.rs:352`. I believe calling `deserialize_str()` instead would work correctly.

## `cargo test` Output
```
% cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests (target/debug/deps/rspotify_serde_json_bug-1b96b40ad66c2775)

running 2 tests
test test::slice ... ok
test test::reader ... FAILED

failures:

---- test::reader stdout ----
thread 'test::reader' panicked at 'called `Result::unwrap()` on an `Err` value: Error("invalid type: string \"averyrealplaylistid\", expected a borrowed string", line: 1, column: 28)', src/main.rs:17:60
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::reader

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--bin rspotify-serde-json-bug'
```