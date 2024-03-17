# texclean
Recursively clean all LaTeX projects in a given directory

# Usage

```bash
>texclean --help
Recursively clean all LaTeX projects in a given directory that match the specified criteria

Usage: texclean [OPTIONS]

Options:
  -d, --directory <DIR>  The directory in which the projects will be searched [default: .]
  -s, --simulate         Perform a trial run with no changes made
  -h, --help             Print help
  -V, --version          Print version
```

Just run `texclean` to recursively search for all potential latex auxiliary files (`.aux`, `.bbl`, `.blg`, `.log`, latexindent files, latexmk files, synctex files etc.) from the working directory downward and interactively delete them. Pass another directory to search in using `--directory`. Use `texclean --simulate` flag if you want to do a dry-run without any actual deletions.

# Limitations and possible improvements

As of right now the functionality is pretty bare bones because I just quickly threw everything together to where it was "useful enough" for me. Some potential improvements include:

* improving "latex project detection" to decrease the number of false positives (for example by searching for potentially related `.tex` or `.bib` files). Currently a lot of non-latex `.log` files might be included for example
* adding additional files to remove (I think minted generates some directories that people might want to remove for example)

# Installation

You can install texclean using [cargo](https://github.com/rust-lang/cargo) (which you can in turn install using [rustup](https://rustup.rs/))

```bash
cargo install texclean
```

If the install fails (shouldn't happen, but sometimes it happens) try `cargo install --locked texclean` instead to build with versions exactly as specified in the lockfile. 