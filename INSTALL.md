<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# Installation

eza is available for Windows, macOS and Linux.

### Cargo (crates.io)

![Crates.io](https://img.shields.io/crates/v/eza?link=https%3A%2F%2Fcrates.io%2Fcrates%2Feza)

If you already have a Rust environment set up, you can use the `cargo install` command:

    cargo install eza

Cargo will build the `eza` binary and place it in your `CARGO_INSTALL_ROOT`.
For more details on installation location see [the cargo
book](https://doc.rust-lang.org/cargo/commands/cargo-install.html#description).

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

First make sure you have the `gpg` command, and otherwise install it via:

```bash
sudo apt update
sudo apt install -y gpg
```

Then install eza via:

```bash
sudo mkdir -p /etc/apt/keyrings
wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | sudo gpg --dearmor -o /etc/apt/keyrings/gierens.gpg
echo "deb [signed-by=/etc/apt/keyrings/gierens.gpg] http://deb.gierens.de stable main" | sudo tee /etc/apt/sources.list.d/gierens.list
sudo chmod 644 /etc/apt/keyrings/gierens.gpg /etc/apt/sources.list.d/gierens.list
sudo apt update
sudo apt install -y eza
```
_Note_: In strict apt environments, you may need to add the target: `echo "deb [arch=amd64 signed-by=...` 

### Nix (Linux, MacOS)

[![nixpkgs unstable package](https://repology.org/badge/version-for-repo/nix_unstable/eza.svg)](https://repology.org/project/eza/versions)

> **Note**
> Installing packages imperatively isn't idiomatic Nix, as this can lead to [many issues](https://stop-using-nix-env.privatevoid.net/).

Eza is available from [Nixpkgs](https://github.com/NixOS/nixpkgs) and from the
flake in this repository.

For `nix profile` users:

```shell
nix profile install nixpkgs#eza
```

For `nix-env` users:

```shell
nix-env -i eza
```

**Declarative Nix Installations**

- Simple NixOS installation: [rfaulhaber/dotfiles](https://github.com/rfaulhaber/dotfiles/blob/a8d084d178efd0592b7ac02d34a450fb58913aca/nix/modules/programs/eza/default.nix#L15)
- Using the flake via NixOS: [hallettj/home.nix](https://github.com/hallettj/home.nix/blob/a8388483e5d78e110be73c5af0e7f0e3ca8f8aa3/flake.nix#L19)
- Using home-manager on NixOS: [Misterio77/nix-config](https://github.com/Misterio77/nix-config/blob/6867d66a2fe7899c608b9c8e5a8f9aee279d188b/home/misterio/features/cli/fish.nix#L6)

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

[![Fedora package](https://repology.org/badge/version-for-repo/fedora_39/rust:eza.svg)](https://repology.org/project/eza/versions)

> ⚠️ **Note:** As of **Fedora 42**, `eza` is **no longer available** in the official Fedora repositories due to the absence of an active maintainer.
>
> If you're using Fedora 42 or newer, consider one of these options:
>
> - **Use a pre-built binary** from the [Releases](https://github.com/eza-community/eza/releases) page
> - **Build from source** by following the [Cargo (git)](#cargo-git) instructions above
>
> 💬 Interested in helping? [Become a Fedora package maintainer](https://docs.fedoraproject.org/en-US/package-maintainers/) or reach out via [Matrix](https://matrix.to/#/#eza-community:gitter.im).

For Fedora versions **prior to 42**, `eza` is available in the official repository:

```bash
sudo dnf install eza
```

### Void Linux

[![Void Linux package](https://repology.org/badge/version-for-repo/void_x86_64/eza.svg)](https://repology.org/project/eza/versions)

Eza is available as the [eza](https://github.com/void-linux/void-packages/tree/master/srcpkgs/eza) package in the official Void Linux repository.

```bash
sudo xbps-install eza
```

### Termux

Eza is available as the [eza](https://github.com/termux/termux-packages/tree/master/packages/eza) package in the official Termux repository.

```bash
pkg install eza
```

### Manual (Linux)

Example is for x86_64 GNU, replaces the file names if downloading for a different arch.

```shell
wget -c https://github.com/eza-community/eza/releases/latest/download/eza_x86_64-unknown-linux-gnu.tar.gz -O - | tar xz
sudo chmod +x eza
sudo chown root:root eza
sudo mv eza /usr/local/bin/eza
```

If `exa` was installed before, replace it with `eza`:
```shell
sudo rm -f /usr/local/bin/exa
sudo ln -s /usr/local/bin/eza /usr/local/bin/exa
```

### Pixi (Linux, MacOS, and Windows)

[![conda-forge package](https://img.shields.io/conda/vn/conda-forge/eza)](https://prefix.dev/channels/conda-forge/packages/eza)

Eza is available in the conda-forge repository and can be installed using [Pixi](https://pixi.sh/latest/):

```shell
pixi global install eza
```

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

### Flox (Linux, macOS, Windows WSL)

Eza is available from [Flox](https://flox.dev).

To install eza, run:

```shell
flox install eza
```

### X-CMD (Linux, macOS, Windows WSL, Windows GitBash)

Eza is available from [x-cmd](https://www.x-cmd.com).

To install eza, run:

```shell
x env use eza
# or
x eza
```

### fox (Linux, macOS)

Eza is available from [fox](https://www.getfox.sh/).

To install eza, run:

```shell
fox install eza
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


#### For zsh with homebrew:

In case zsh completions don't work out of the box with homebrew, add the
following to your `~/.zshrc`:

```bash
if type brew &>/dev/null; then
    FPATH="$(brew --prefix)/share/zsh/site-functions:${FPATH}"
    autoload -Uz compinit
    compinit
fi
```

For reference:
- https://docs.brew.sh/Shell-Completion#configuring-completions-in-zsh
- https://github.com/Homebrew/brew/issues/8984
