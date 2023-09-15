<div align="center">

# eza

eza is a modern, maintained replacement for ls, built on [exa](https://github.com/ogham/exa).

**README Sections:** [Options](#options) — [Installation](#installation) — [Development](#development)

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

<a href="https://matrix.to/#/#eza-community:gitter.im"><img alt="Gitter" src="https://img.shields.io/gitter/room/eza-community/eza?logo=element&link=https%3A%2F%2Fapp.gitter.im%2F%23%2Froom%2F%23eza%3Agitter.im&link=Gitter%20matrix%20room%20for%20Eza" width=200></a>

[![Unit tests](https://github.com/eza-community/eza/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/eza-community/eza/actions/workflows/unit-tests.yml)
![Crates.io](https://img.shields.io/crates/v/eza?link=https%3A%2F%2Fcrates.io%2Fcrates%2Feza)
![Crates.io](https://img.shields.io/crates/l/eza?link=https%3A%2F%2Fgithub.com%2Feza-community%2Feza%2Fblob%2Fmain%2FLICENCE)

</div>

![Screenshots of eza](screenshots.png)

---

**eza** is a modern, maintained replacement for the venerable file-listing command-line program `ls` that ships with Unix and Linux operating systems, giving it more features and better defaults.
It uses colours to distinguish file types and metadata.
It knows about symlinks, extended attributes, and Git.
And it’s **small**, **fast**, and just **one single binary**.

By deliberately making some decisions differently, eza attempts to be a more featureful, more user-friendly version of `ls`.

---

**eza** features not in exa (non-exhaustive):

- Fixes [“The Grid Bug”](https://github.com/eza-community/eza/issues/66#issuecomment-1656758327) introduced in exa 2021.
- Hyperlink support.
- Mount point details.
- Selinux context output.
- Git repo status output.
- Human readable relative dates.
- Several security fixes.
- Support for `bright` terminal colours.
- Many smaller bug fixes/changes!

---

<a id="try-it">
<h1>Try it!</h1>
</a>

### Nix ❄️

If you already have Nix setup with flake support, you can try out eza with the `nix run` command:

    nix run github:eza-community/eza

Nix will build eza and run it.

If you want to pass arguments this way, use e.g. `nix run github:eza-community/eza -- -ol`.

<a id="installation">
<h1>Installation</h1>
</a>

eza is available for Windows, macOS and Linux.

### Cargo (crates.io)

![Crates.io](https://img.shields.io/crates/v/eza?link=https%3A%2F%2Fcrates.io%2Fcrates%2Feza)

If you already have a Rust environment set up, you can use the `cargo install` command:

    cargo install eza

Cargo will build the `eza` binary and place it in `$HOME/.local/share/cargo/bin/eza`.

### Cargo (git)

If you already have a Rust environment set up, you can use the `cargo install` command in your local clone of the repo:

    git clone https://github.com/eza-community/eza.git
    cd eza
    cargo install --path .

Cargo will build the `eza` binary and place it in `$HOME/.cargo`.

### Arch Linux

[![Arch Linux package](https://repology.org/badge/version-for-repo/arch/eza.svg)](https://repology.org/project/eza/versions)

Eza is available in the [\[extra\]](https://archlinux.org/packages/extra/x86_64/eza/) repository of Arch Linux.

```bash
pacman -S eza
```

### Debian and Ubuntu

Eza is available from [deb.gierens.de](http://deb.gierens.de). The GPG public
key is in this repo under [deb.asc](/deb.asc).

To install eza from this repo use:

```bash
wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | sudo tee /etc/apt/trusted.gpg.d/gierens.asc
echo "deb http://deb.gierens.de stable main" | sudo tee /etc/apt/sources.list.d/gierens.list
sudo apt update
sudo apt install -y eza
```

**Note:** on some systems like Docker containers apt might require the key
to be in dearmored format, then use this command instead:

```bash
wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/gierens.asc
```

before proceeding with the others from above.

### Nix (Linux, MacOS)

[![nixpkgs unstable package](https://repology.org/badge/version-for-repo/nix_unstable/eza.svg)](https://repology.org/project/eza/versions)

Eza is available from [Nixpkgs](https://github.com/NixOS/nixpkgs).

For `nix profile` users:

```shell
nix profile install nixpkgs#eza
```

For `nix-env` users:

```shell
nix-env -i eza
```

### Gentoo

[![Gentoo package](https://repology.org/badge/version-for-repo/gentoo/eza.svg)](https://repology.org/project/eza/versions)

On Gentoo, eza is available as a package [`sys-apps/eza`](https://packages.gentoo.org/packages/sys-apps/eza):

```bash
emerge --ask sys-apps/eza
```

### openSUSE

Eza is available at [openSUSE:Factory/eza](https://build.opensuse.org/package/show/openSUSE:Factory/eza):

```bash
zypper ar https://download.opensuse.org/tumbleweed/repo/oss/ factory-oss
zypper in eza
```

The preceding repository also contains the Bash, Fish, and Zsh completions.

### Fedora

Fedora support is in the works.

https://bugzilla.redhat.com/show_bug.cgi?id=2238264

### Brew (MacOS)

[![Homebrew package](https://repology.org/badge/version-for-repo/homebrew/eza.svg)](https://repology.org/project/eza/versions)

Eza is available from [Homebrew](https://formulae.brew.sh/formula/eza#default).

To install eza, run:

```shell
brew install eza
```

### MacPorts (macOS)

[![MacPorts port](https://repology.org/badge/version-for-repo/macports/eza.svg)](https://repology.org/project/eza/versions)

On macOS, eza is also available via [MacPorts](https://ports.macports.org/port/eza/).

To install eza, run:

```shell
sudo port install eza
```

### Winget (Windows)

[![Windows package](https://repology.org/badge/version-for-repo/winget/eza.svg)](https://repology.org/project/eza/versions)


Eza is available on Winget.

To install eza, run:

```shell
winget install eza-community.eza
```

### Scoop (Windows)

[![Windows package](https://repology.org/badge/version-for-repo/scoop/eza.svg)](https://repology.org/project/eza/versions)

Eza is available from [Scoop](https://scoop.sh/#/apps?q=eza&id=a52070d25f94bbcc884f80bef53eb47ed1268198).

To install eza, run:

```shell
scoop install eza
```

### Completions

#### For zsh:

> **Note**
> Change `~/.zshrc` to your preferred zsh config file.

##### Clone the repository:
   
```sh
git clone https://github.com/eza-community/eza.git
```

##### Add the completion path to your zsh configuration:
   
Replace `<path_to_eza>` with the actual path where you cloned the `eza` repository.

```sh
echo 'export FPATH="<path_to_eza>/completions/zsh:$FPATH"' >> ~/.zshrc
```

##### Reload your zsh configuration:
   
```sh
source ~/.zshrc
```

---

Click sections to expand.

<a id="options">
<details>
    <summary> Command-line options </summary>

<h1>Command-line options</h1>
</a>

eza’s options are almost, but not quite, entirely unlike `ls`’s.

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
- **--hyperlink**: display entries as hyperlinks
- **-w**, **--width=(columns)**: set screen width in columns

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
- **-M**, **--mounts**: Show mount details (Linux only).
- **-S**, **--blocksize**: show size of allocated file system blocks
- **-t**, **--time=(field)**: which timestamp field to use
- **-u**, **--accessed**: use the accessed timestamp field
- **-U**, **--created**: use the created timestamp field
- **-X**, **--dereference**: dereference symlinks for file information
- **-Z**, **--context**: list each file’s security context
- **-@**, **--extended**: list each file’s extended attributes and sizes
- **--changed**: use the changed timestamp field
- **--git**: list each file’s Git status, if tracked or ignored
- **--no-git**: suppress Git status (always overrides `--git`, `--git-repos`, `--git-repos-no-status`)
- **--time-style**: how to format timestamps
- **--no-permissions**: suppress the permissions field
- **-o**, **--octal-permissions**: list each file's permission in octal format
- **--no-filesize**: suppress the filesize field
- **--no-user**: suppress the user field
- **--no-time**: suppress the time field

Some of the options accept parameters:

- Valid **--color** options are **always**, **automatic**, and **never**.
- Valid sort fields are **accessed**, **changed**, **created**, **extension**, **Extension**, **inode**, **modified**, **name**, **Name**, **size**, **type**, and **none**. Fields starting with a capital letter sort uppercase before lowercase. The modified field has the aliases **date**, **time**, and **newest**, while its reverse has the aliases **age** and **oldest**.
- Valid time fields are **modified**, **changed**, **accessed**, and **created**.
- Valid time styles are **default**, **iso**, **long-iso**, **full-iso**, and **relative**.

</details>

<a id="development">
<details>
    <summary> Development </summary>
<h1>Development

<a href="https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html">
    <img src="https://img.shields.io/badge/rustc-1.70.0+-lightgray.svg" alt="Rust 1.70.0" />
</a>

</h1></a>

eza is written in [Rust](https://www.rust-lang.org/).
You will need rustc version 1.56.1 or higher.
The recommended way to install Rust for development is from the [official download page](https://www.rust-lang.org/tools/install), using rustup.

Once Rust is installed, you can compile eza with Cargo:

    cargo build
    cargo test

- The [just](https://github.com/casey/just) command runner can be used to run some helpful development commands, in a manner similar to `make`.
  Run `just --list` to get an overview of what’s available.

- If you are compiling a copy for yourself, be sure to run `cargo build --release` or `just build-release` to benefit from release-mode optimisations.
  Copy the resulting binary, which will be in the `target/release` directory, into a folder in your `$PATH`.
  `/usr/local/bin` is usually a good choice.

- To compile and install the manual pages, you will need [pandoc](https://pandoc.org/).
  The `just man` command will compile the Markdown into manual pages, which it will place in the `target/man` directory.
  To use them, copy them into a directory that `man` will read.
  `/usr/local/share/man` is usually a good choice.

- eza depends on [libgit2](https://github.com/rust-lang/git2-rs) for certain features.
  If you’re unable to compile libgit2, you can opt out of Git support by running `cargo build --no-default-features`.

- If you intend to compile for musl, you will need to use the flag `vendored-openssl` if you want to get the Git feature working.
  The full command is `cargo build --release --target=x86_64-unknown-linux-musl --features vendored-openssl,git`.

### Developing on Nix (experimental) ❄️

If you have a working Nix installation with flake support, you can use nix to manage your dev environment.

    nix develop

The Nix Flake has a few features:

- Run `nix flake check` to run `treefmt` on the repo.
- Run `nix build` and manually test `./results/bin/eza -- <arguments>` for easy debugging.
- Run `nix build .#test` to run `cargo test` via the flake.
- Run `nix build .#clippy` to lint with clippy (still work in progress).

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=eza-community/eza&type=Date)](https://star-history.com/#eza-community/eza&Date)
