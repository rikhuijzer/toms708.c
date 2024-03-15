**THIS IS STILL A WORK IN PROGRESS. DON'T SUBMIT PULL REQUESTS. I'LL PROBABLY WON'T MERGE THEM.**

# toms708.c

Compute the Incomplete Beta Function ratio Ix(a, b).

The file `toms708.c` contains a stand-alone C implementation of the TOMS708 algorithm.
`toms708` was originally a Fortran77 program by Armido Didonato and Alfred Morris, Jr.
It was later updated by John Burkardt and converted to a C file for the R project.

This repository contains an updated version of R's C file which aims to be more readable:

- Replace most unstructured control flow, such as GOTOs and macros, with structured control flow (functions) or just replace them by the code they would have executed.
- Reformatting the code via `clang-format`.
- Move functions around so that function calls are always after function definition.

Unlike many other files in the R `nmath` directory, the `toms708.c` file does not mention the GNU General Public License in its header.
Therefore I changed the license for this C file to one of the most permissive licenses: MIT.

## Notes on Replacing GOTOs with Functions

When jumping from one location in the code to another, all variables that were in scope will remain in scope.
This means that, say, a variable `int x` is in scope when `goto foo` is called, then `x` will be in scope during the execution of `foo`.
To replace this `goto` with a function, we need to pass `x` as an argument to `foo`.
This can become very complex when there are many variables used inside the `goto` block.

Luckily, modern C tooling can help us with this.
When having set up `clangd` with inline code hints, it will automatically verify that:

- All variables that are used inside the newly created function are available.
- The variable names passed into the function are the same as the variable names in the function signature.
    This ensures that the code is executed with the same variables as before.

It will not verify whether the code would enter the labeled block without a `goto`.
This is something that needs to be done manually.

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

The tests compare the output of the C code with the output of `bpbeta` from the R language.
