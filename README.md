# 🚀 rscopy

**A lightning-fast 🦀 Rust implementation of macOS's `pbcopy`!**

---

## ✨ What is `rscopy`?

`rscopy` is a simple, no-fuss command-line tool that copies input from **standard input (stdin)** straight to your system clipboard 📋 — just like `pbcopy`, but written in Rust for speed ⚡️ and reliability 💪.

---

## 🔧 How it works

```bash
echo "Hello from Rust!" | rscopy
```

Whatever you pipe into `rscopy` will instantly be available on your clipboard, ready to paste wherever you like 🚀.

---

## 🛠️ Installation

### 🐣 Build from source

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install) 🦀, then:

```bash
git clone https://github.com/yourusername/rscopy.git
cd rscopy
cargo build --release
```

Then copy the binary to somewhere in your `PATH`:

```bash
cp target/release/rscopy /usr/local/bin/
```

✨ Done!

---

## 🧪 Usage

| Command                  | What it does                             |
| ------------------------ | ---------------------------------------- |
| `echo "text" \| rscopy`  | Copies `"text"` to your clipboard ✍️     |
| `cat file.txt \| rscopy` | Copies the contents of `file.txt` 📄     |
| `rscopy < file.txt`      | Another way to copy a file's contents 📂 |

Example:

```bash
echo "Copy this to clipboard!" | rscopy
```

---

## 📦 Dependencies

* [`clipboard`](https://crates.io/crates/clipboard) crate — handles the clipboard operations for you.

---
