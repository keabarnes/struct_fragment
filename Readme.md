# StructFragment

A macro to create a duplicate of a struct with keys removed.

I mainly developed this macro to help with creating structs for interacting with the `diesel` create:

- original struct is the model, for reads from the database
- a struct with `id` removed, used for inserts into the database
- [coming soon] the ability to do this multiple times, allowing the definition of a struct fragment which would be received as a `POST` body.

## Usage

Either providing a list of ignored fields:

```rust
#[derive(StructFragment)]
#[fragment_ignore_list = "id,updated_at"]
#[fragment_name = "DbUser"]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub updated_at: DateTime
}
```

Or adding `#[fragment_ignore]` to fields you don't want included.

```rust
#[derive(StructFragment)]
#[fragment_name = "DbUser"]
pub struct User {
    #[fragment_ignore] pub id: i32,
    pub full_name: String,
    pub email: String,
    #[fragment_ignore] pub updated_at: DateTime
}
```

## Useful tools

- Provides the final expanded result of a macro, good for visualising what's actually happening: https://github.com/dtolnay/cargo-expand
