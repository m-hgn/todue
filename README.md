
# ☑️  todue

Keep track of TODOs and deadlines in your terminal

---

### Example


```
    - [x] Take out the trash
    - [ ] Discard of the body
    - [ ] Do the dishes
```

---

### Installation

Make sure you have a working [rust environment](https://rustup.rs/)!

Then execute the following commands in order.
This assumes `/usr/bin` is on your `$PATH`:

1.
    Clone the repository.
    ```sh
    git clone https://gitlab.com/m-hgn/todue.git
    ```
2.
    Build the release executable.
    ```sh
    cd todue && cargo build --release
    ```
3.
    Install the binary
    ```sh
    sudo cp target/release/todue /usr/bin/todue
    ```
4.
    Optional clean up
    ```sh
    cd .. && rm -rf todue
    ```

