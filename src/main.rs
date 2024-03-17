use clap::builder::OsStr;
use clap::{self, arg, command, value_parser};
use spinoff::{spinners, Spinner};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct Pathingy(DirEntry);

impl std::fmt::Display for Pathingy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.path().display())
    }
}

impl AsRef<Path> for Pathingy {
    fn as_ref(&self) -> &Path {
        self.0.path()
    }
}

fn main() {
    let matches = command!()
        .about("Recursively clean all LaTeX projects in a given directory that match the specified criteria")
        .args(&[
            arg!(
                -d --directory <DIR> "The directory in which the projects will be searched"
            )
            .default_value(OsStr::from("."))
            .value_parser(value_parser!(PathBuf)),
            arg!(
                -s --simulate "Perform a trial run with no changes made"
            ),
        ])
        .get_matches();

    let directory = matches.get_one::<PathBuf>("directory").unwrap(); // default value is set - unwrap is safe
    let dry_run = matches.get_flag("simulate");

    const SPINNER_STYLE: spinners::Arc = spinners::Arc;
    let spinner = RefCell::new(Spinner::new(SPINNER_STYLE, "Loading...", None));
    let mut last = Instant::now();
    let aux_file_candidates = WalkDir::new(&directory)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| {
            if let Some(d) = e.ok() {
                let now = Instant::now();
                if (now - last) > Duration::from_millis(1000) && d.file_type().is_dir() {
                    let mut spinner = spinner.borrow_mut();
                    spinner.update(
                        SPINNER_STYLE,
                        format!("Searching {}", d.path().display()),
                        None,
                    );
                    last = now;
                }
                Some(d)
            } else {
                None
            }
        })
        .filter(|e| is_latex_artifact(e.path()))
        .map(Pathingy)
        .collect();
    let mut spinner = spinner.borrow_mut();
    spinner.success("Done!");
    let selected = inquire::MultiSelect::new("Select files to delete", aux_file_candidates)
        .with_page_size(15)
        .prompt()
        .unwrap();

    if inquire::Confirm::new("Confirm deletion of selected elements [y/N]")
        .with_default(false)
        .prompt_skippable()
        .unwrap()
        .unwrap_or(false)
    {
        for path in selected {
            if !dry_run {
                println!("Removing {}", path);
                std::fs::remove_file(path).unwrap()
            } else {
                println!("Would remove {}", path);
            }
        }
    } else {
        println!("Did not delete any files")
    }
}

fn is_latex_artifact<P: AsRef<Path>>(file: P) -> bool {
    let path = file.as_ref();
    // yes I should probably turn at least some of this into a regex or 2 but eh
    path.is_file()
        && (path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|extension| match extension {
                "aux" | "bbl" | "blg" | "log" | "out" | "toc" | "fdb_latexmk" | "soc" => true,
                _ => false,
            })
            || path
                .file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|stem| stem.starts_with("__latexindent"))
            || path
                .file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|name| name.ends_with(".run.xml") || name.ends_with(".synctex.gz")))
}
