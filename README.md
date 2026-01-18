# 🌳 AVL Dictionary Library (Rust + C API)

A dictionary project based on an **AVL Tree** (balanced binary search tree). The library is written in **Rust** with a focus on manual memory management (strictly avoiding `std::Box`, `std::String`, `std::Vec`) and exposes a native API for the **C** language.

## 🚀 Features

* **Data Structure:** AVL Tree
* **Memory Management:** Manual allocation and deallocation using `libc`.
* **Custom Types:** `MyString` implementation replacing the standard `String`.
* **Operations:**
  * `insert` - Adding and updating elements.
  * `find` - Searching for values by key.
  * `contains` - Checking for key existence.
  * `delete` - Removing elements with automatic tree rebalancing.


* **Interfaces:**
* Rust: Safe API + `dict!` macro.
* C: `extern "C"` functions operating on raw pointers.



## 📂 Project Structure

```text
dictionary_project/
├── Cargo.toml          # Project configuration
├── Cargo.lock
├── dictionary.h        # C header file
├── main.c              # Example program in C
├── examples/
│   └── demo.rs         # Example program in Rust
└── src/
    ├── lib.rs          # Main library module
    ├── c_api.rs        # C compatibility layer (FFI)
    ├── tree.rs         # AVL tree logic
    ├── mem.rs          # Wrappers for libc::malloc/free
    └── mystring.rs     # Custom string implementation
```

## 🛠️ Requirements

* **Rust** (Cargo).
* **C Compiler** (MSVC tools recommended on Windows: `cl.exe` available via "x64 Native Tools Command Prompt for VS").

---

## ⚙️ Build and Run Instructions

### 1. `Cargo.toml` Configuration

To ensure the project works as both a C library and a Rust crate, verify that your `Cargo.toml` contains the following section:

```toml
[lib]
crate-type = ["staticlib", "rlib"]
```

### 2. Rust (Tests and Demo)

To run unit tests (verifying rotations and logic):

```bash
cargo test
```

To run the example Rust program (demonstrating the macro and all functions):

```bash
cargo run --example demo
```

### 3. C Language (Integration)

To compile and run the `main.c` program using the library, follow these steps in the **x64 Native Tools Command Prompt for VS**.

**Step A: Build the static library (.lib)**

```cmd
cargo build
```

*This will create a `.lib` file in the `target/debug/` directory.*

**Step B: Compile the C program**
Use the `cl.exe` compiler, linking your project library along with the required Windows system libraries (`ws2_32`, `ntdll`, etc.).

*Note: The command below assumes your project name in Cargo.toml is `project_custom_avl_dictionary`. If it is different, update the .lib filename accordingly.*

```cmd
cl main.c /Fe:demo_c.exe /link /LIBPATH:target\debug project_custom_avl_dictionary.lib ws2_32.lib advapi32.lib userenv.lib bcrypt.lib ntdll.lib
```

**Step C: Run the program**

```cmd
demo_c.exe
```

## 📝 Implementation Notes

* The project strictly avoids `unwrap` and `panic` in production code (replaced by `match` and safe error handling).
* Functions are modularized to ensure no function exceeds 30 lines of code.
* The tree correctly handles memory cleanup (`dict_clear`), recursively freeing all nodes and allocated strings.