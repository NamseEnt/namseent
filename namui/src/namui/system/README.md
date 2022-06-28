# How to add system for new platform

- if system file is `abc.rs`, make directory `abc` and branch them into platform directory or file

ex)

```
// before
- abc.rs

// after (1)
- abc
  - web.rs
  - windows.rs

// after (2)

- abc
  - web
    - mod.rs
  - windows.rs
```
