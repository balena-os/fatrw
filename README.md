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

The internals of FATRW will be illustrated by running it in `--debug` mode.

Let's create a `content.txt` file inside a `/test` folder with some sample content:

```text
$ echo "Sample content" | fatrw --debug write content.txt
1   Write content.txt
2   Absolute /test/content.txt
3   Content MD5 checksum e8f62ec15eff2ece315ee7e692ffa648
4   Create .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum
5   Fsync /test/.content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum
6   Fsync /test
7   Committing checksum file
8   Checksum verified
9   Copy .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp
10  Fsync /test/.content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp
11  Fsync /test
12  Rename .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp content.txt
13  Fsync /test/content.txt
14  Fsync /test
15  Remove .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum
16  Fsync /test
```

(3) Calculate the MD5 checksum of the provided content. In the example above, the content is `Sample content`, and its checksum is `e8f62ec15eff2ece315ee7e692ffa648`.

(4) Create a file with the given content in the same folder where the `content.txt` target file should be created. Include the checksum in the filename along with an 8-symbol random string. The random string is necessary to prevent race conditions in case multiple processes are writing the same file with identical content.

(5) When a file is created, issue an `fsync` on its file descriptor.

(6) Also trigger an `fsync` on the parent `/test` folder. According to the `fsync` man page: "Calling fsync() does not necessarily ensure that the entry in the directory containing the file has also reached disk. For that an explicit fsync() on a file descriptor for the directory is also needed."

At this point, if a power cycle occurs before the target file is fully written, the file can be restored from the md5sum file. This is crucial for the operation of FATRW.

(9) Copy the `md5sum` file to a `tmp` file, which will be later renamed to the target `content.txt` file in step (12).

(15) Remove the `md5sum` file since, at this point, it is guaranteed that the target file has been successfully written to the disk.

In the provided example, if a power cycle occurs during steps (12), (13), or (14), the `content.txt` file may end up damaged and truncated. However, at that point, it is guaranteed that the `md5sum` temporary file will exist next to it, and upon subsequent boot and read operations, the `content.txt` file will be properly restored:

```text
$ fatrw --debug read content.txt
1   Read content.txt
2   Absolute /test/content.txt
3   Parent directory /test
4   Glob pattern /test/.content.txt.*.*.md5sum
5   Found .md5sum file .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum
6   Committing checksum file
7   Checksum verified
8   Copy .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp
9   Fsync /test/.content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp
10  Fsync /test
11  Rename .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.tmp content.txt
12  Fsync /test/content.txt
13  Fsync /test
14  Remove .content.txt.cfa9c461.e8f62ec15eff2ece315ee7e692ffa648.md5sum
15  Fsync /test
16  Md5sum file committed
Sample content
```

(4) Check for the existence of `md5sum` files corresponding to the `content.txt` file.

(5) Found is an `md5sum` file indicating a previous abruptly terminated write of the `content.txt` file.

(6) Initiate the process of restoring `content.txt` from the `md5sum` file.

(7) Verify the content of the `md5sum` file against the MD5 checksum stored as part of the filename of the checksum file.

(8) Copy the `md5sum` file to a `tmp` file that will be later renamed to the target `content.txt` file in step (11).

(14) Remove the `md5sum` file as, at this point, the `content.txt` file has been successfully restored.

The content of the `content.txt` file can be returned successfully at the end.
