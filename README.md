
# 🐍 snakify

Command line utility to quickly snake-caseify any input text.

---

### Example

Quickly create source file from copied text

```sh
$ snakify 1879 Minimum XOR Sum of Two Arrays.rs | xargs touch

$ ls
total 0
0 ./  0 ../  0 1879_minimum_xor_sum_of_two_arrays.rs*

```

---

### Installation

Make sure you have a working [rust environment](https://rustup.rs/),
that supports [`#![feature(iter_intersperse)]`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse)

Then execute the following commands in order.
This assumes `/usr/bin` is on your `$PATH`:

1.
    Clone the repository.
    ```sh
    git clone https://gitlab.com/m-hgn/snakify.git
    ```
2.
    Build the release executable.
    ```sh
    cd snakify && cargo build --release
    ```
3.
    Install the binary
    ```sh
    sudo cp target/release/snakify-rs /usr/bin/snakify
    ```
4.
    Optional clean up
    ```sh
    cd .. && rm -rf snakify
    ```

