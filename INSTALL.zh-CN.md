<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# 安装

eza 支持 Windows、macOS 与 Linux。

> **语言**：[English](https://github.com/eza-community/eza/blob/main/INSTALL.md) | 简体中文（本文档）
>
> 本文档为 eza 官方 INSTALL.md 的简体中文译本。所有命令、软件包名、路径与链接均保持原样未译。

### Cargo（crates.io）

![Crates.io](https://img.shields.io/crates/v/eza?link=https%3A%2F%2Fcrates.io%2Fcrates%2Feza)

如果你已经配置好 Rust 环境，可以使用 `cargo install` 命令：

    cargo install eza

Cargo 会构建 `eza` 二进制文件，并将其放置在你的 `CARGO_INSTALL_ROOT` 中。
有关安装位置的更多细节，请参阅 [cargo 手册](https://doc.rust-lang.org/cargo/commands/cargo-install.html#description)。

### Cargo（git）

如果你已经配置好 Rust 环境，可以在本仓库的本地克隆中使用 `cargo install` 命令：

    git clone https://github.com/eza-community/eza.git
    cd eza
    cargo install --path .

Cargo 会构建 `eza` 二进制文件，并将其放置在 `$HOME/.cargo` 中。

### Arch Linux

[![Arch Linux package](https://repology.org/badge/version-for-repo/arch/eza.svg)](https://repology.org/project/eza/versions)

Eza 已收录于 Arch Linux 的 [\[extra\]](https://archlinux.org/packages/extra/x86_64/eza/) 仓库中。

```bash
pacman -S eza
```

### Debian 与 Ubuntu

Eza 可从 [deb.gierens.de](http://deb.gierens.de) 获取。其 GPG 公钥位于本仓库的 [deb.asc](/deb.asc)。

首先请确保系统中已有 `gpg` 命令，否则请先安装它：

```bash
sudo apt update
sudo apt install -y gpg
```

然后通过以下方式安装 eza：

```bash
sudo mkdir -p /etc/apt/keyrings
wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | sudo gpg --dearmor -o /etc/apt/keyrings/gierens.gpg
echo "deb [signed-by=/etc/apt/keyrings/gierens.gpg] http://deb.gierens.de stable main" | sudo tee /etc/apt/sources.list.d/gierens.list
sudo chmod 644 /etc/apt/keyrings/gierens.gpg /etc/apt/sources.list.d/gierens.list
sudo apt update
sudo apt install -y eza
```
_注意_：在较为严格的 apt 环境中，你可能还需要加上目标架构参数：`echo "deb [arch=amd64 signed-by=...`

### Nix（Linux、macOS）

[![nixpkgs unstable package](https://repology.org/badge/version-for-repo/nix_unstable/eza.svg)](https://repology.org/project/eza/versions)

> **注意**
> 以命令式（imperative）方式安装软件包并不符合 Nix 的惯用法，因为这可能导致[诸多问题](https://stop-using-nix-env.privatevoid.net/)。

Eza 可从 [Nixpkgs](https://github.com/NixOS/nixpkgs) 以及本仓库中的 flake 获取。

适用于 `nix profile` 用户：

```shell
nix profile install nixpkgs#eza
```

适用于 `nix-env` 用户：

```shell
nix-env -i eza
```

**声明式（Declarative）Nix 安装**

- 简易 NixOS 安装：[rfaulhaber/dotfiles](https://github.com/rfaulhaber/dotfiles/blob/a8d084d178efd0592b7ac02d34a450fb58913aca/nix/modules/programs/eza/default.nix#L15)
- 通过 flake 在 NixOS 上使用：[hallettj/home.nix](https://github.com/hallettj/home.nix/blob/a8388483e5d78e110be73c5af0e7f0e3ca8f8aa3/flake.nix#L19)
- 在 NixOS 上使用 home-manager：[Misterio77/nix-config](https://github.com/Misterio77/nix-config/blob/6867d66a2fe7899c608b9c8e5a8f9aee279d188b/home/misterio/features/cli/fish.nix#L6)

### Gentoo

[![Gentoo package](https://repology.org/badge/version-for-repo/gentoo/eza.svg)](https://repology.org/project/eza/versions)

在 Gentoo 上，eza 以软件包 [`sys-apps/eza`](https://packages.gentoo.org/packages/sys-apps/eza) 的形式提供：

```bash
emerge --ask sys-apps/eza
```

### openSUSE

Eza 可从 [openSUSE:Factory/eza](https://build.opensuse.org/package/show/openSUSE:Factory/eza) 获取：

```bash
zypper ar https://download.opensuse.org/tumbleweed/repo/oss/ factory-oss
zypper in eza
```

上述仓库中同时提供了 Bash、Fish 与 Zsh 的命令补全脚本。

### Fedora

[![Fedora package](https://repology.org/badge/version-for-repo/fedora_44/rust:eza.svg)](https://repology.org/project/eza/versions)

Eza 以 Fedora 官方仓库中的 [eza](https://packages.fedoraproject.org/pkgs/rust-eza/eza/) 软件包形式提供。

```bash
sudo dnf install eza
```

### Void Linux

[![Void Linux package](https://repology.org/badge/version-for-repo/void_x86_64/eza.svg)](https://repology.org/project/eza/versions)

Eza 以 Void Linux 官方仓库中的 [eza](https://github.com/void-linux/void-packages/tree/master/srcpkgs/eza) 软件包形式提供。

```bash
sudo xbps-install eza
```

### Termux

Eza 以 Termux 官方仓库中的 [eza](https://github.com/termux/termux-packages/tree/master/packages/eza) 软件包形式提供。

```bash
pkg install eza
```

### 手动安装（Linux）

以下示例针对 x86_64 GNU 平台，若下载其他架构的版本请替换相应文件名。

```shell
wget -c https://github.com/eza-community/eza/releases/latest/download/eza_x86_64-unknown-linux-gnu.tar.gz -O - | tar xz
sudo chmod +x eza
sudo chown root:root eza
sudo mv eza /usr/local/bin/eza
```

如果此前已安装 `exa`，可用 `eza` 替换它：
```shell
sudo rm -f /usr/local/bin/exa
sudo ln -s /usr/local/bin/eza /usr/local/bin/exa
```

### Pixi（Linux、macOS 与 Windows）

[![conda-forge package](https://img.shields.io/conda/vn/conda-forge/eza)](https://prefix.dev/channels/conda-forge/packages/eza)

Eza 收录于 conda-forge 仓库，可通过 [Pixi](https://pixi.sh/latest/) 安装：

```shell
pixi global install eza
```

### Brew（macOS）

[![Homebrew package](https://repology.org/badge/version-for-repo/homebrew/eza.svg)](https://repology.org/project/eza/versions)

Eza 可从 [Homebrew](https://formulae.brew.sh/formula/eza#default) 获取。

安装 eza，请运行：

```shell
brew install eza
```

### MacPorts（macOS）

[![MacPorts port](https://repology.org/badge/version-for-repo/macports/eza.svg)](https://repology.org/project/eza/versions)

在 macOS 上，eza 也可以通过 [MacPorts](https://ports.macports.org/port/eza/) 获取。

安装 eza，请运行：

```shell
sudo port install eza
```

### Winget（Windows）

[![Windows package](https://repology.org/badge/version-for-repo/winget/eza.svg)](https://repology.org/project/eza/versions)

Eza 已上架 Winget。

安装 eza，请运行：

```shell
winget install eza-community.eza
```

### Scoop（Windows）

[![Windows package](https://repology.org/badge/version-for-repo/scoop/eza.svg)](https://repology.org/project/eza/versions)

Eza 可从 [Scoop](https://scoop.sh/#/apps?q=eza&id=a52070d25f94bbcc884f80bef53eb47ed1268198) 获取。

安装 eza，请运行：

```shell
scoop install eza
```

### Flox（Linux、macOS、Windows WSL）

Eza 可从 [Flox](https://flox.dev) 获取。

安装 eza，请运行：

```shell
flox install eza
```

### X-CMD（Linux、macOS、Windows WSL、Windows GitBash）

Eza 可从 [x-cmd](https://www.x-cmd.com) 获取。

安装 eza，请运行：

```shell
x env use eza
# 或
x eza
```

### fox（Linux、macOS）

Eza 可从 [fox](https://www.getfox.sh/) 获取。

安装 eza，请运行：

```shell
fox install eza
```

### 命令补全

#### 针对 zsh：

> **注意**
> 请将 `~/.zshrc` 替换为你实际使用的 zsh 配置文件。

##### 克隆仓库：

```sh
git clone https://github.com/eza-community/eza.git
```

##### 将补全脚本路径加入 zsh 配置：

请将 `<path_to_eza>` 替换为你克隆 `eza` 仓库的实际路径。

```sh
echo 'export FPATH="<path_to_eza>/completions/zsh:$FPATH"' >> ~/.zshrc
```

##### 重新加载 zsh 配置：

```sh
source ~/.zshrc
```


#### 针对配合 homebrew 使用的 zsh：

如果通过 homebrew 安装后 zsh 补全未能开箱即用，请将
以下内容加入你的 `~/.zshrc`：

```bash
if type brew &>/dev/null; then
    FPATH="$(brew --prefix)/share/zsh/site-functions:${FPATH}"
    autoload -Uz compinit
    compinit
fi
```

参考资料：
- https://docs.brew.sh/Shell-Completion#configuring-completions-in-zsh
- https://github.com/Homebrew/brew/issues/8984
