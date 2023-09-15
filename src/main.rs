use litemap::LiteMap;

use clap::{Command, CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};

#[cfg(unix)]
use walkdir::WalkDir;

use std::fs;
use std::io;
use std::process;

use humansize::{format_size, BINARY, DECIMAL};

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None,arg_required_else_help(true))]
struct Args {
    #[clap(help="Specify directories to check for largest files", value_hint = ValueHint::DirPath)]
    dirs: Vec<String>,
    #[clap(default_value_t = 10, short = 'n', help = "Number of files to display")]
    numfiles: usize,
    #[clap(short = 'X', help = "Don't descend into other file systems")]
    xdev: bool,
    #[clap(short = 'G', help = "Show sizes in powers of ten")]
    gigabyte: bool,
    #[clap(long = "skip-hidden", help = "Skip hidden files and directories")]
    skip_hidden: bool,
    #[clap(long = "generate-completion", group = "only_one", value_enum)]
    generator: Option<Shell>,
}

pub fn exists_dir(file: &str) -> bool {
    match fs::metadata(file) {
        Ok(attr) => attr.is_dir(),
        Err(_) => false,
    }
}
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let args = Args::parse();

    if let Some(generator) = args.generator {
        let mut cmd = Args::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
        process::exit(0)
    }
    process::exit(search_directory_tree(&args));
}

struct FileSizes {
    // contains the largest files found
    // we have Vec<String< because it could happen
    // that there is more than a single file with
    // a certain size
    fsmap: LiteMap<usize, Vec<String>>,
    // the smallest file we have in the LiteMap
    smallest: usize,
    // the number of files we have in the fsmap
    numfiles: usize,
    // maximum number of largest files to keep
    maxfiles: usize,
}

impl FileSizes {
    fn new(maxfiles: usize) -> FileSizes {
        FileSizes {
            fsmap: LiteMap::new(),
            smallest: 0,
            numfiles: 0,
            maxfiles,
        }
    }

    fn add_file(&mut self, sz: usize, filename: &str) {
        // first we fill the list of largest files according to self.maxfiles
        if self.numfiles < self.maxfiles {
            if self.numfiles == 0 {
                self.smallest = sz;
            }
            if let Some(v) = self.fsmap.get_mut(&sz) {
                v.push(filename.to_string());
            } else {
                self.fsmap.insert(sz, vec![filename.to_string()]);
                self.numfiles += 1;
            }

            if sz < self.smallest {
                self.smallest = sz;
            }

            return;
        }

        // if a file is smaller than the smallest of the
        // already found files just ignore it
        if self.numfiles >= self.maxfiles && sz < self.smallest {
            return;
        }
        if sz >= self.smallest {
            if let Some(v) = self.fsmap.get_mut(&sz) {
                // we have already a file with size `sz`
                // so we just add the file to that existing entry
                v.push(filename.to_string());
            } else {
                // here we add a new file into the LiteMap
                self.fsmap.insert(sz, vec![filename.to_string()]);
                self.numfiles += 1;
                // which means we remove the smallest already existing
                // value
                if self.numfiles > self.maxfiles {
                    if let Some(_v) = self.fsmap.remove(&self.smallest) {
                        // now find the new smallest key and update
                        // self.smallest accordingly
                        if let Some((k, _v)) = self.fsmap.first() {
                            self.smallest = *k;
                        }
                    }
                }
            }
        }
    }

    fn show_results(&mut self, gigabyte: bool) {
        for (key, value) in self.fsmap.iter_mut() {
            let fkey = if gigabyte {
                format_size(*key, DECIMAL)
            } else {
                format_size(*key, BINARY)
            };
            println!("{:>10} {}", fkey, value[0]);

            // remove first value
            // if there are more files with the same size
            // print them indented
            let _ = value.remove(0);
            for v in value {
                println!("{:>11}{}", ' ', v);
            }
        }
    }
}

fn search_directory_tree(args: &Args) -> i32 {
    println!(
        "TOP{} Finding the {} largest files in given directories",
        args.numfiles, args.numfiles
    );
    let mut filesizes = FileSizes::new(args.numfiles);
    let mut error_happened = 0;

    for dir in &args.dirs {
        if !exists_dir(dir) {
            println!("Directory {} does not exist. Exiting...", dir);
            process::exit(1);
        };

        for dir_entry in WalkDir::new(dir)
            .same_file_system(args.xdev)
            .follow_links(false)
        {
            match dir_entry {
                Ok(dir_entry) => {
                    let file_name = dir_entry.file_name().to_string_lossy();
                    if args.skip_hidden && file_name.starts_with('.') {
                        continue;
                    }
                    match dir_entry.metadata() {
                        Ok(md) => {
                            if !md.is_file() {
                                continue;
                            }
                            filesizes.add_file(
                                md.len().try_into().unwrap(),
                                &dir_entry.path().to_string_lossy(),
                            );
                            //                            filesizes.add_file(md.len(), &dir_entry.path().to_string_lossy());
                        }
                        Err(e) => println!("Error retrieving metadata: {}", e),
                    };
                }
                Err(e) => {
                    eprintln!("{}", e);
                    error_happened += 1;
                }
            };
        }
    }
    filesizes.show_results(args.gigabyte);
    error_happened
}
