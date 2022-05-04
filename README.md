# Find largest files in directory trees

`flf` is a simple to use program to find the largest files in one or more directory trees.

`flf`, by default, shows the 10 largest files in the specified directory tree(s).
If for a certain size there exists multiple files then the output would show
more than 10 files. 

*Important:* `fzf` doesn't check if files with the same size are hardlinks.

Other similar utilities

- https://github.com/noahrinehart/lrg


# Usage

## Getting help information

Run `flf --help` to get help.

```
flf 0.1.0
Manfred Lotz <manfred.lotz@posteo.de>
Find largest files in directory trees.

USAGE:
    flf [OPTIONS] [DIRS]...

ARGS:
    <DIRS>...    Specify directories to check for largest files

OPTIONS:
    -G                                       Show sizes in powers of ten
        --generate-completion <GENERATOR>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                               Print help information
    -n <NUMFILES>                            Number of files to display [default: 10]
        --skip-hidden                        Skip hidden files and directories
    -V, --version                            Print version information
    -X                                       Don't descend into other file systems
```

## Find largest files

Examples

```
flf /usr/share/man

flf $HOME/Downloads $HOME/Documents
```

```
flf -n 5 /usr/share/man                                                                                 10:49:38
```

```
TOP5 Finding the 5 largest files in given directories
 319.80 KiB /usr/share/man/man1/ffplay-all.1.gz
 322.55 KiB /usr/share/man/man1/ffprobe-all.1.gz
 328.10 KiB /usr/share/man/man1/x86_64-linux-gnu-g++-9.1.gz
            /usr/share/man/man1/x86_64-linux-gnu-gcc-9.1.gz
 360.00 KiB /usr/share/man/man1/x86_64-linux-gnu-g++-11.1.gz
            /usr/share/man/man1/x86_64-linux-gnu-gcc-11.1.gz
 409.48 KiB /usr/share/man/man1/ffmpeg-all.1.gz
```

## Show a specific number of largest files

Example

```
flf -n 5 /usr/share/man
```

## Don't descend into other file systems


Example

```
flf -X /data/docs
```

## Install completion for your shell

Example for fish:

````
flf --generate-completion fish > ~/.config/fish/completions/flf.fish
````

