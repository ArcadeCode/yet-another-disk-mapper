# Yet Another Disk Mapper (YADM)

**YADM** is a lightweight disk mapping tool designed for small-scale projects.  
It allows you to serialize the structure of a file system directory into compact and optimized formats for fast storage and access.

---

## 🚀 Features

- Generates lightweight and ultra-optimized mapping files.
- Provides serializers for Rust to handle `.msgpack.tar.gz` files.

---

## ⚙️ How It Works

Running a command like `./yadm.exe serialize "C://"` will perform the following steps:

1. Traverse the specified directory and collect all file and folder metadata.
2. Convert the resulting data structure into a [MessagePack](https://msgpack.org/) object.
3. Compress the MessagePack file using `tar.gz`.
4. Output the final mapping as `map.msgpack.tar.gz`.

In my personal configuration :
- SSD NVMe
- 471,45 Go on 500,00 Go used
- Output result :
    ```
    1. Scanning folders at: C://
    Scan Time: 195.4473487s
    Elements found: 2065488
    2. Parsing to hashmap...
    3. Encoding + writing MessagePack...

    --- Résultats ---
    Compression Time: 5.5725601s
    Average compression time per file: 2697 ns
    Compressed file size: 358444787 octets
    Compressed file size: 350043.74 Ko
    Compressed file size: 341.84 Mo
    Average compressed size per file: 173 octets
    ```
---

## 🧰 Commands

There are two main commands:

- `yadm serialize "path" --use-tar-gz=true`  
  → Maps a directory and outputs a compressed `.msgpack.tar.gz` file.

- `yadm serialize "path" --use-tar-gz=false`  
  → Outputs a raw `.msgpack` file (uncompressed).

- `yadm parse "map.msgpack.tar.gz"`  
  → Parses the mapping and loads it into memory as a `Vec<HashMap<String, String>>`.

---

## 🦀 Usage in Rust

You can integrate YADM directly in your Rust project like this:

```rust
use yadm::{serialize, parse};
use std::collections::HashMap;

fn main() {
    // Serialize a directory
    serialize("C://", true);

    // Parse the resulting file
    let map: Vec<HashMap<String, String>> = parse("map.msgpack.tar.gz");

    // You can now use `map` in your application
}
```
> ⚠️ Make sure to handle Result and error types appropriately in real applications.