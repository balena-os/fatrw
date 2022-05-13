## FATRW - Safe read and write for FAT

FATRW is a CLI utility and a Rust library that implements safe file read and write operations for FAT file systems. FAT does not support atomic I/O operations thus a power cycle during a file write may leave it in a damaged state.

FATRW provides safe file read and write operations that overcome the lack of atomic I/O by saving checksums in separate temporary hidden files in the same directory a file is written. Upon successful completion of a write command the temporary checksums will be deleted. If during a read operation such temporary checksums are detected this will indicate that the original file could be in a damaged state and respective measures will be taken. See [How it works](#how-it-works) for more information.

### How to use

Please note that the read and write commands have to be always used in conjunction with one another - a file witten by FATRW has to be read with FATRW.

To write a file use a Unix pipe:

```sh
$ echo '{"persistentLogging": true' | fatrw write config.json`
```

To write a file with permissions set those in numeric mode:

```sh
$ echo '{"persistentLogging": true}' | fatrw write -m 644 config.json
```

To read a file:

```sh
$ fatrw read config.json
{"persistentLogging": true}
```

To copy a file:

```sh
$ fatrw copy source.json dest.json
```

### Rust library

To write a file:

```rust
import fatrw

fatrw.write_file("data.txt", "content", None);
```

To write a file with permissions append those in numeric mode:

```rust
import fatrw

fatrw.write_file("data.txt", "content", Some(644));
```

To read a file:

```rust
import fatrw

let content = fatrw.read_file("/mnt/boot/config.json");
```

To copy a file:

```rust
import fatrw

fatrw.copy_file("/mnt/data/source.json", "/mnt/boot/dest.json");
```

### How it works

TODO
