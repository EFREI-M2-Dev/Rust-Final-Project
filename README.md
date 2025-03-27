# 🦀 Rust Projet Final

## Installation

```bash
git clone https://github.com/EFREI-M2-Dev/Rust-Final-Project.git
cd Rust-Final-Project
bash setup_git_hook.sh
```

---

## Commands

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

## Debug

To enable debugging, follow these steps:

1. **Open two terminals**:

    - In the **first terminal**, execute the `tty` command to identify the correct terminal path.
    - Copy the value of the terminal path from the output and paste it into the `tty_path` field in `config.toml`.

2. **Execute the program**:

    - In the **second terminal**, run the program as usual by executing:

        ```
        cargo run
        ```

    This will ensure the application sends debug messages to the specified terminal.

## Changelog

To generate the changelog:

```sh
sh generate_changelog.sh 1.0.0
```

> You need to verify and adjust the content of the changelog.