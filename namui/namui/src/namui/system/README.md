# How to add system for new platform

## file or directory

if system file is `abc.rs`, make directory `abc` and branch them into platform directory or file

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

## init

Don't forget make init fn and call it in mod.rs
