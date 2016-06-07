# ceph-safe-disk
Ceph disk safety removal tool

# Building
Builds on stable `rustc` as of 1.8

# Using
```
Usage: ceph-safe-disk [OPTION]

Options:
    -h, --help          Print help information
    -q, --quick         Give a quick, non-exhaustive status of removable OSDs

Exit statuses:
    1: General error
    2: Safe to remove an OSD
    3: Not safe to remove an OSD
```
