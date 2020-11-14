# My design doc v.0
## How the log is structured

### LogEntry
The initial version of log is struct contains three field
```rust
struct LogEntry{
    id: usize,
    key: String,
    value: String,
}
```
Log will be stored in files names like `__NUM.cbor`, starts from `__0.cbor`. Each
log will store `SINGLE_FILE_LOG_ENTRY` constant lines of log entries.

CBOR stands for `Concise Binary Object Representation` from [RFC 7049](https://tools.ietf.org/html/rfc7049).

The library we used are [Serde CBOR](https://github.com/pyfisch/cbor).


> TODO
> 1. What metadata we can store to improve the performance
> 2. Why limited to 10000 lines? Current decision is kind arbitrary.


### LogPointer
Currently, the log pointer will only contains two number, file id and line id.
```rust
struct LogPointer{
    f_id : usize,
    l_id : usize,
}
```

### In memory index
The in memory index will keep track of current keys value pairs in the KvStore.
In our case, we will use a single hashmap to keep track of this.
```rust
HashMap<String, LogPointer>
```

When we want to close the database, we will need to save the in memory index
into a file. We will store the metadata in an `__index.cbor`. For now, we will
just marshal our in memory data structure into a toml file, but we can change
this format later.

### Conclusion
The current design might not be a good design since it needs to read multiple
files if there are many entries. It might be a good idea to store all the
entries in a single file, just like sqlite. However, to do that, I might need to
redesign the file storage, which I will do it in the next revision.