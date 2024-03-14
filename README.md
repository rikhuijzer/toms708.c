# toms708

Computes the Incomplete Beta Function ratio Ix(a,b).

`toms708` was originally a Fortran77 program by Armido Didonato and Alfred Morris, Jr.
It was later updated by John Burkardt and converted to a C file for the R project.

This repository contains an updated version of R's C file which has been cleaned up:

- The code has been reformatted to be more readable.
- Most unstructured control flow, such as GOTOs and macros, have been replaced with structured control flow (functions).

The main aim of this project is to make the code more readable by making the control flow and scope of variables more clear.

## Testing

To test the code, install a recent Rust version and run:

```sh
$ cargo test --manifest-path='tests/Cargo.toml'
```

Or, to watch for changes run:

```sh
$ cd tests/

$ cargo watch -x test
```
