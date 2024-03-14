# toms708.c

Compute the Incomplete Beta Function ratio Ix(a, b).

The file `toms708.c` contains a stand-alone C implementation of the TOMS708 algorithm.
`toms708` was originally a Fortran77 program by Armido Didonato and Alfred Morris, Jr.
It was later updated by John Burkardt and converted to a C file for the R project.

This repository contains an updated version of R's C file which has been cleaned up:

- The code has been reformatted to be more readable.
- Most unstructured control flow, such as GOTOs and macros, have been replaced with structured control flow (functions).

The main aim of this project is to make the code more readable by making the control flow and scope of variables more clear.

Unlike many other files in the R `nmath` directory, the `toms708.c` file does not mention the GNU General Public License in its header.
Therefore I changed the license to one of the most permissive licenses: MIT.

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
