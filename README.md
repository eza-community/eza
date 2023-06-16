<div align="center">

# zetta

zetta builds on the awesome foundation of [exa](https://the.exa.website/) so we 
can add more features. When exa becomes maintained again, features could be
merged back.

For simplicity, only installation method for now is `cargo install`. 

**this readme will be extended again, pending some rewrites / updates.** 

![Screenshots of zetta](screenshots.png)

---

**zetta** is a modern replacement for the venerable file-listing command-line program `ls` that ships with Unix and Linux operating systems, giving it more features and better defaults.
It uses colours to distinguish file types and metadata.
It knows about symlinks, extended attributes, and Git.
And it’s **small**, **fast**, and just **one single binary**.

By deliberately making some decisions differently, zetta attempts to be a more featureful, more user-friendly version of `ls`.


---

<a id="options">
<h1>Command-line options</h1>
</a>

zetta’s options are almost, but not quite, entirely unlike `ls`’s.

### Display options

- **-1**, **--oneline**: display one entry per line
- **-G**, **--grid**: display entries as a grid (default)
- **-l**, **--long**: display extended details and attributes
- **-R**, **--recurse**: recurse into directories
- **-T**, **--tree**: recurse into directories as a tree
- **-x**, **--across**: sort the grid across, rather than downwards
- **-F**, **--classify**: display type indicator by file names
- **--colo[u]r**: when to use terminal colours
- **--colo[u]r-scale**: highlight levels of file sizes distinctly
- **--icons**: display icons
- **--no-icons**: don't display icons (always overrides --icons)

### Filtering options

- **-a**, **--all**: show hidden and 'dot' files
- **-d**, **--list-dirs**: list directories like regular files
- **-L**, **--level=(depth)**: limit the depth of recursion
- **-r**, **--reverse**: reverse the sort order
- **-s**, **--sort=(field)**: which field to sort by
- **--group-directories-first**: list directories before other files
- **-D**, **--only-dirs**: list only directories
- **--git-ignore**: ignore files mentioned in `.gitignore`
- **-I**, **--ignore-glob=(globs)**: glob patterns (pipe-separated) of files to ignore

Pass the `--all` option twice to also show the `.` and `..` directories.

### Long view options

These options are available when running with `--long` (`-l`):

- **-b**, **--binary**: list file sizes with binary prefixes
- **-B**, **--bytes**: list file sizes in bytes, without any prefixes
- **-g**, **--group**: list each file’s group
- **-h**, **--header**: add a header row to each column
- **-H**, **--links**: list each file’s number of hard links
- **-i**, **--inode**: list each file’s inode number
- **-m**, **--modified**: use the modified timestamp field
- **-S**, **--blocks**: list each file’s number of file system blocks
- **-t**, **--time=(field)**: which timestamp field to use
- **-u**, **--accessed**: use the accessed timestamp field
- **-U**, **--created**: use the created timestamp field
- **-@**, **--extended**: list each file’s extended attributes and sizes
- **--changed**: use the changed timestamp field
- **--git**: list each file’s Git status, if tracked or ignored
- **--time-style**: how to format timestamps
- **--no-permissions**: suppress the permissions field
- **--octal-permissions**: list each file's permission in octal format
- **--no-filesize**: suppress the filesize field
- **--no-user**: suppress the user field
- **--no-time**: suppress the time field

Some of the options accept parameters:

- Valid **--color** options are **always**, **automatic**, and **never**.
- Valid sort fields are **accessed**, **changed**, **created**, **extension**, **Extension**, **inode**, **modified**, **name**, **Name**, **size**, **type**, and **none**. Fields starting with a capital letter sort uppercase before lowercase. The modified field has the aliases **date**, **time**, and **newest**, while its reverse has the aliases **age** and **oldest**.
- Valid time fields are **modified**, **changed**, **accessed**, and **created**.
- Valid time styles are **default**, **iso**, **long-iso**, **full-iso**, and **relative**.


---

<a id="installation">
<h1>Installation</h1>
</a>

zetta is available for macOS and Linux.

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

    cargo install zetta

Cargo will build the `zetta` binary and place it in `$HOME/.cargo`.

To build without Git support, run `cargo install --no-default-features zetta` is also available, if the requisite dependencies are not installed.
