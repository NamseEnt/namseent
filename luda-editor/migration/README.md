# How to use migration?

## 1. Use `#[migration::version({version})]` on your struct

- Version should be start from 1.
- For the first version, don't put attribute. The migration system will be check that is 0 version.

```rust
// This is your first version of struct
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
}

// And this is your next version. use version 1.
#[migration::version(1)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
}
```

## 2. Define `migrate(prev) -> Self` function

- This macro will call `YourStruct::migrate(previous_struct) -> YourStruct` to migrate previous struct to next version.
- So, define `migrate(prev) -> Self` function for it.

```rust

mod first_version {
    // This is your first version of struct
    #[derive(Debug, Clone, Default)]
    pub struct Cut {
        id: Uuid,
    }
}


// And this is your next version. use version 1.
#[migration::version(1)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
}

impl Cut {
    pub fn migrate(previous: first_version::Cut) -> Self {
        Cut {
            id: previous.id,
            ...
        }
    }
}
```
