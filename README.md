# toms708.c

Compute the Incomplete Beta Function ratio Ix(a, b).

The file `toms708.c` contains a stand-alone C implementation of the TOMS708 algorithm.
`toms708` was originally a Fortran77 program by Armido Didonato and Alfred Morris, Jr.
It was later updated by John Burkardt and converted to a C file for the R project.

This repository contains an updated version of R's C file which aim to be more readable:

- The code has been reformatted via `clang-format`.
- Most unstructured control flow, such as GOTOs and macros, have been replaced with structured control flow (functions) or just replaced by the code they would have executed.

Unlike many other files in the R `nmath` directory, the `toms708.c` file does not mention the GNU General Public License in its header.
Therefore I changed the license for this C file to one of the most permissive licenses: MIT.

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
