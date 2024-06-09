# Rust Clap Example

Examples that demonstrate some advanced features in Rust ([Macros](https://doc.rust-lang.org/reference/macros.html),  [Attributes](https://doc.rust-lang.org/reference/attributes.html)) using [clap](https://crates.io/crates/clap) crate.
## Installation

1. **Clone the repository:**

    ```sh
    git clone https://github.com/ZenGo-X/rust-clap-example.git
    cd rust-clap-example
    ```

2. **Select which example to run in the `main.rs`**:
    ```rust
    fn main() {
        macros_example::run_example();
    }
    ```
   or
    ```rust
    fn main() {
        attributes_example::run_example();
    }
    ```

3. **Build the project:**

    ```sh
    cargo build
    ```

4. **Run the executable:**

    ```sh
    ./target/debug/rust-clap-example
    ```
