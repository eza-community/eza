<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# 为 eza 贡献

> **语言**：[English](https://github.com/eza-community/eza/blob/main/CONTRIBUTING.md) | 简体中文（本文档）
>
> 本译文中的命令、文件路径、` conventional commits ` 类型关键字（如 `feat`、`fix`）均保持原文。

如果你想为 eza 贡献代码，有几件事是你务必要先熟悉的。

- 行为准则：[贡献者公约行为准则](CODE_OF_CONDUCT.zh-CN.md)
- 必须符合 [Conventional Commits（约定式提交）](https://www.conventionalcommits.org/zh-hans/v1.0.0/) 规范
- 必须符合 [Semantic Versioning（语义化版本）](https://semver.org/lang/zh-CN/) 规范
- [安全策略](SECURITY.zh-CN.md)
- [自由与开放源代码（FOSS）软件](https://www.gnu.org/philosophy/free-sw.html)

## eza 开发环境

强烈建议你安装 Nix 来进行 eza 开发。我们借助
Nix 来轻松地测试 eza 并将其分发给众多用户，同时它让我们
能够方便地为开发者提供多种工具。有了 Nix，你就不必逐个
手动安装依赖并搭建开发环境，而是可以直接使用与本项目开发者完全一致的环境。

因此，安装 Nix 并启用 flakes 这一“实验性”特性会很有帮助。此外，为了让自己的
eza 开发过程尽可能轻松，安装
[direnv](https://direnv.net/) 会让你省心不少。

当你进入 eza 仓库目录时，如果你已安装 `direnv`，系统会提示你通过 `direnv allow` 授权。
这样做可以免去每次打开该文件夹都要手动进入开发环境的麻烦。
如果你没有安装 direnv，也可以用 `nix develop` 临时应急，以进入该 direnv 环境。

该开发环境包含了对约定式提交的基本合规检查、cargo clippy 静态检查等众多功能。

其中还包含一个 pre-commit 钩子，可以大幅降低犯下那些无谓拖延 PR 被合入的错误。
最重要的是，它能确保你的提交符合约定式提交规范。

一些常用命令包括：
- `nix flake check`：检查一切是否正确。
- `nix build`：构建 eza。
- `nix build .#test`：运行 eza 的 cargo 测试
- `nix build .#clippy`：对 eza 运行 clippy
- `nix fmt`：按 flake 检查与
  pre-commit-hooks.nix 的要求自动格式化你的代码
- `just itest`：运行集成测试

**对于不使用 Nix 的用户，**
仓库中也提供了传统的 `pre-commit` 钩子，你可以通过系统包管理器安装，或用
`brew|pip install pre-commit` 安装，然后在仓库根目录运行 `pre-commit install -c .pre-commit-config-non-nix.yaml`。
之后这些钩子会在你提交时自动运行。

[just](https://github.com/casey/just) 这个命令运行器可用于执行一些
有用的开发命令，用法类似于 `make`。运行 `just --list`
可查看所有可用命令的概览。

要编译手册页，你需要 [pandoc](https://pandoc.org/)，nix flake 应当已经为你安装好了。
`just man` 命令会把 Markdown 编译为手册页，并将其放在 `target/man` 目录中。

eza 的某些功能依赖 [libgit2](https://github.com/rust-lang/git2-rs)。
如果你无法编译 libgit2，可以通过运行 `cargo build --no-default-features` 来关闭 Git 支持。
同样地，nix flake 应当已经为你处理好了这个问题；如果没有，请提交一个 issue。

如果你打算针对 musl 进行编译，并且希望 Git 功能可用，则需要使用
`vendored-openssl` 标志。完整的命令是
`cargo build --release --target=x86_64-unknown-linux-musl --features
vendored-openssl,git`。

如果你想了解关于测试的更多信息，请阅读：
[测试 eza](https://github.com/eza-community/eza/blob/main/TESTING.md)（中译：[TESTING.zh-CN.md](TESTING.zh-CN.md)）

## 创建 PR

首先，请使用 PR 模板。

请确保你所做的事情……真的能跑通。同时也请在 PR 描述中
说明你是如何验证这一点的。此外，还需要你尽最大努力检查是否存在回归问题。

如果你的 PR 引入了一个标志（flag），你必须：
- 为 bash、zsh、fish、nushell 添加命令补全
- 在手册页中补充文档
- 将该选项加入帮助（help）输出
- 将该选项加入 README.md

在提交之前，你应当已经运行过 `nix flake check`，并确保所有问题都已解决。
对于格式问题，`nix fmt` 会为你格式化代码。大多数 clippy 问题都可以通过
`cargo clippy --fix` 解决（不过自己动手修复可能更有学习价值）。
如果你遇到 reuse 相关问题，可以运行下面的命令来标注你的代码：

以下是最基础的要求：
- 你的提交摘要必须符合约定式提交规范。
- 你的提交应当拆分成较小的、逻辑清晰的块。
- 评审者可能会要求你 rebase，把提交重组成更合理的块。
- 你的 PR 必须通过 CI 以及本地的 `cargo test`。
- 评审者可能会要求你重构部分代码。

请记住，这里的任何人都不是雇员，请按照行为准则的要求尊重每一个人。
也请记住，如果你的 PR 需要一段时间才有回复，请保持耐心。通常情况下不会太久，
但每个人一天的时间都有限，只要有可能我们都会做到零延迟。
而“延迟”本身就足以说明它是必要的。

## 提交信息
一条常见的提交信息至少应包含摘要，以及在有对应 issue 时的
关闭动作引用，还可以包含描述与签名。

### 摘要
对于你的提交信息，请在第一行简要概括该提交所改动的内容。
尽量保持在 72 字符以内，并在前面加上改动类型。可参考以下列表：
- feat: 为 eza 新增一项特性
- feat(zsh): 为 zsh 补全新增内容
- refactor: 修订部分代码
- docs(readme): 修订 README
- docs(man): 修订手册页
- fix: 修复代码库中的缺陷
- fix(ci): 修复持续集成中的缺陷
- ...

请注意，该列表并不完整，而且某些提交可能同时符合多种类型，
因此请尽量做出最恰当的判断。这能为维护者合并你的 PR 省下大量工作。

### 描述
如果由于改动复杂或需要外部信息才能使提交易于理解，那么
你应该补充更详细的改动说明、理由，并在必要时附上外部文档链接。
该描述应位于摘要下方空两行处，除链接外请保持在 80 字符以内。

### Issue 引用
如果该提交解决了某个 issue，请添加：`Resolves: #abc`，其中 `abc` 为 issue 编号。
如果是缺陷修复，也可以使用 `Fixes: #abc`。

### 签名
你可以在末尾处、描述或 issue 引用的下方空两行添加签名。

### 示例
以下是一条遵循上述规则的破坏性变更提交信息示例：

```
fix(hyperlinks)!: TextCell building of detailed grid view, hyperlink, icon options

The hyperlink option adds an escape sequence which in the normal TextCell
creation also becomes part of the length calculation. This patch applies
the same logic the normal grid already did, by using the filenames bare
width when a hyperlink is embedded. It also respects the ShowIcons
option just like the normal grid view.

BREAKING CHANGE: The style codes for huge file and units where
documented to be `nt` and `ut` but the code was using `nh` and `uh`.
The code has been updated to match the documented style codes.
EXA_COLORS using style codes `nh` and `uh` will need to be updated to
use `nt` and `ut`.

Resolves: #129
Ref: #473, #319

Co-authored-by: 9glenda <plan9git@proton.me>
Signed-off-by: Christina Sørensen <christina@cafkafk.com>
```

### 更多示例

- feat: add column selection
- fix(output): fix width issue with columns
- test(fs): add tests for filesystem metadata
- feat!: breaking change / feat(config)!: implement config file
- chore(deps): update dependencies

### 提交类型

- build：影响构建系统或外部依赖的改动（例如 libgit2）
- ci：对 CI 配置文件与脚本的改动（示例 scope：Nix、Vagrant、Docker）
- chore：不改变源代码或测试的改动（例如：对构建过程、辅助工具、库的改动）
- docs：仅涉及文档、README、命令补全、手册页
- feat：新特性
- fix：缺陷修复
- perf：用于提升或解决性能问题的代码改动
- refactor：既不修复缺陷也不新增特性的代码改动
- revert：回滚某项改动
- style：不影响代码含义的改动（例如 clippy）
- test：补充缺失的测试或修正已有测试
