# PNG resizer

This is a small command line utility to resize PNGs written in Rust. I'm writing it to teach myself Rust and especially the [Rust FFI](http://doc.rust-lang.org/guide-ffi.html).

The program currently crashes. This is yet to be resolved.

# Compiling

```
cargo build
```

# Running
```
cargo run in.png out.png
```

This will make `in.png` two times larger and save it to `out.png`. The scaling factor is hardcoded for now while I solve other problems with the program.
