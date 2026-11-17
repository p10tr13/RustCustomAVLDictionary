# AVL Dictionary in Rust with a C API

[![CI](https://github.com/p10tr13/RustCustomAVLDictionary/actions/workflows/ci.yml/badge.svg)](https://github.com/p10tr13/RustCustomAVLDictionary/actions/workflows/ci.yml)

A dictionary from `u64` keys to strings, built on a self-balancing **AVL tree** and written in Rust **without `Box`, `Vec` or `String`**: every node and every string is allocated and freed by hand with `libc::malloc` and `libc::free`. The library has a safe Rust API on top of the raw tree and exports a C interface, so it can be linked into a C program.

The memory handling is checked in CI by [Miri](https://github.com/rust-lang/miri), which interprets the test suite and reports undefined behaviour in `unsafe` code (use-after-free, double free, invalid pointers, leaks), and by Valgrind on the C demo.

## Features

- **AVL tree**: insert, find, contains and delete in `O(log n)`, with LL, RR, LR and RL rotations and rebalancing after both insertion and deletion.
- **Manual memory management**: nodes and strings live in `malloc`ed memory; `MyString` replaces `String`.
- **Safe Rust API**: `Dictionary` owns the tree and frees it when dropped, so callers write no `unsafe` code. The `dictionary!` macro builds one from `key => value` pairs.
- **Low-level Rust API**: the `tree` module works on raw node pointers, and the `dict!` macro returns a raw root.
- **C API**: `extern "C"` functions declared in `custom_avl_dictionary.h`.
- **No panics in the library code**: `unwrap` and `panic!` are avoided in favour of explicit error handling, and no function is longer than 30 lines.

## Usage

### Rust

```rust
use project_custom_avl_dictionary::{dictionary, Dictionary};

let mut dict = dictionary! { 1 => "a", 2 => "b", 3 => "c" };
dict.insert(2, "bb");                  // replaces the value
assert_eq!(dict.get(2), Some("bb"));
assert!(dict.contains(1));
assert!(dict.remove(3));
assert_eq!(dict.len(), 2);
// the tree is freed here, when `dict` goes out of scope
```

### C

```c
#include "custom_avl_dictionary.h"

Dict* root = dict();
dict_insert(&root, 1, "a");
dict_insert(&root, 2, "b");
const char* value = dict_find(root, 2);   /* "b", or NULL if the key is missing */
dict_delete(&root, 1);
dict_clear(root);                         /* frees every node */
```

The pointer returned by `dict_find` stays valid until that key is deleted or the dictionary is cleared.

## Building and running

Requires Rust 1.85 or newer (edition 2024).

```bash
cargo test                      # unit tests and randomized tests against BTreeMap
cargo run --example demo        # Rust demo: macro, rotations, deletions, tree printout
cargo +nightly miri test        # undefined-behaviour check (needs `rustup component add miri`)
```

### C demo

`cargo build --release` produces a static library in `target/release/`.

Linux or macOS:

```bash
cargo build --release
gcc -o demo_c main.c -Ltarget/release -lproject_custom_avl_dictionary -lpthread -ldl -lm
./demo_c
```

Windows (x64 Native Tools Command Prompt for Visual Studio):

```cmd
cargo build --release
cl main.c /Fe:demo_c.exe /link /LIBPATH:target\release project_custom_avl_dictionary.lib ws2_32.lib advapi32.lib userenv.lib bcrypt.lib ntdll.lib
demo_c.exe
```

## Tests

- **Unit tests** (`src/lib.rs`) cover each rotation, updates, and deletion of leaves, nodes with one child and nodes with two children.
- **Randomized tests** (`tests/dictionary.rs`) run thousands of random inserts and deletes against `std::collections::BTreeMap` and check after every batch that the tree is still ordered and balanced. A second test inserts 100,000 sorted keys and checks that the height stays within the AVL bound of about 1.44 log2 n.
- **CI** runs Clippy with warnings as errors, the tests, both demos, the whole suite under Miri, and the C demo under Valgrind with leak checking.

## Project structure

```
src/
  lib.rs            module setup, dict! macro, unit tests
  dictionary.rs     safe Dictionary wrapper and dictionary! macro
  tree.rs           AVL tree: insert, find, contains, delete, rotations
  mystring.rs       MyString, a malloc-backed string
  mem.rs            typed wrappers around libc::malloc and libc::free
  c_api.rs          extern "C" functions
tests/dictionary.rs randomized tests against BTreeMap
examples/demo.rs    Rust demo
main.c              C demo
custom_avl_dictionary.h   C header
```

## License

[MIT](LICENSE)
