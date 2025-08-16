# fast_cd

`fast_cd` is a small Rust-based tool to quickly jump to frequently used directories using shortcuts. It works like `zoxide` or `autojump`, but with a simple custom database and shell integration.

---

## Features

- Create shortcuts automatically from the last part of a directory path.
- Navigate instantly to directories using short names.
- Stores shortcuts in a simple local database (`~/.fast_cd/data.db`).
- Easy shell integration with a short command (`fcd`) for zsh.

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/anishNagula/fast_cd.git
cd fast_cd
```

2. Build the release binary:
```bash
cargo build --release
```

3. Optionally, move it to a directory in your $PATH:
```bash
mkdir -p ~/.local/bin
cp target/release/fast_cd ~/.local/bin/
```


## Shell Integration (zsh)
Add the following to your ~/.zshrc:

```bash
# Add fast_cd binary to PATH if not already
export PATH="$HOME/.local/bin:$PATH"

# fcd function for fast directory jumps
fcd () {
    local target
    target=$ ("<path...>/fast_cd/target/release/fast_cd" "$1") || return
    cd "$target" || return
}
```

Reload your shell:
```bash
source ~/.zshrc
```

Now you can run:
```bash
fcd <directory_name>
```

- If directory exists in the database, you’ll jump to that directory.
- If not, provided the actual path, it will be added automatically using the last part of the path as the shortcut.

## Usage
```bash
fcd /path/to/your/directory
```

- The last part of the path (e.g., directory) will become the shortcut.
- Future uses of that shortcut will instantly cd to the corresponding path.

## Database
The database is stored at: `~/.fast_cd/data.db`
Format: shortcut:full_path (one per line).
- Automatically created if it does not exist.

## Notes

- Only works in shells where a function like fcd can execute (zsh, bash, etc.).
- Avoid using fc as the function name in zsh because it conflicts with the built-in history command.

## License
MIT License