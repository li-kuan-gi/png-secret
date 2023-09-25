# PNG Secret

Hide your message in a png file.

This project is an implementation of [this guide](https://picklenerd.github.io/pngme_book/) (with some different test codes).

## Install

One can install this binary either 

- Locally
```bash
git clone https://github.com/li-kuan-gi/png-secret
cargo install --path ./png-secret
```

- Remotely
```bash
cargo install --git https://github.com/li-kuan-gi/png-secret
```

## Usage

One can reference the help page to see the usages.
```bash
png-secret -h
```

## Example

- Add a message in some chunk (e.g. `ruSt` chunk)

```bash
png-secret encode <path/to/input.png> ruSt <message> <path/to/output.png>
```

- Show message in some chunk (e.g. `ruSt` chunk)

```bash
png-secret decode <path/to/file.png> ruSt
```

- Remove some chunk (e.g. `ruSt` chunk)

```bash
png-secret remove <path/to/input.png> ruSt <path/to/output.png>
```

- Show all chunks in png file

```bash
png-secret print <path/to/file.png>
```

For valid chunk names, one can reference [the png file spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions)

## Reference

- [picklenerd/pngme_book](https://github.com/picklenerd/pngme_book)

- [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)