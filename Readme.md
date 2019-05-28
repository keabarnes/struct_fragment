**TODO:** 

Improvements include changing this to be closer to what's implemented here:

- https://dev.to/naufraghi/procedural-macro-in-rust-101-k3f
- https://github.com/tylerreisinger/cache-macro/blob/master/src/lib.rs

Also, need to:

- Allow custom naming
- List of fields to delete
- Impl with function from_db_row where it maps from the db struct to the insertable one, like is done with InsertableArtist in kudukudu

**Notes:**

 To help develop:

- https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html
- https://github.com/dtolnay/cargo-expand
