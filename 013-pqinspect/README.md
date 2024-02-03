= Polars and Rust exploration

I've noticed the docs do not contain complete information and sometimes it fails
because it never told you to add the parquet package for example.


== Potentially bugs

* Snappy parquet reading LongUTF8 fails (check parquet file)

```
thread 'main' panicked at /home/marti/.cargo/registry/src/index.crates.io-6f17d22bba15001f/polars-core-0.37.0/src/datatypes/field.rs:172:19:
Arrow datatype Struct([Field { name: "ARA", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "MTE", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "RAC", d
ata_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "RAT", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "TTI", data_type: LargeUtf8, is_nullable: true, metad
ata: {} }, Field { name: "advisory", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "advisory_complement", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name:
 "bytes", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "threat_id_hex", data_type: LargeUtf8, is_nullable: true, metadata: {} }, Field { name: "unix_timestamp", data_type: Flo
at64, is_nullable: true, metadata: {} }, Field { name: "utc", data_type: LargeUtf8, is_nullable: true, metadata: {} }]) not supported by Polars. You probably need to activate that data-type feature.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
