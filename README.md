# Repro steps

```
cargo clean
cargo +1.81.0 build // successful
cargo clean
cargo +1.82.0 build // LNK1181: cannot open input file 'mylib.dll.lib'
```
