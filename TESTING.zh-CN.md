<!--
SPDX-FileCopyrightText: 2024 Christina Sørensen, Martin Fillon
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# 测试 eza

> **语言**：[English](https://github.com/eza-community/eza/blob/main/TESTING.md) | 简体中文（本文档）

## 运行测试

要运行 eza 的测试，你需要准备：
- [just](https://github.com/casey/just)
- [nix](https://nixos.org)

然后任选其一运行：
- `just itest`
- `nix build -L trycmd-local`

## 修改测试

要在 eza 上测试你的改动，在不同情况下你需要做一件或多件事。
你还需要额外准备工具
- [powertest](https://github.com/eza-community/powertest)

如果你想添加一些测试用例，还需要修改 `devtools/dir-generator.sh` 文件。

### 你新增/修改了某个选项

请运行 `just regen` 重新生成 powertesting 用例。然后在 `tests/gen` 或 `tests/cmd` 中查看是否有未通过的测试。

### 你改动了 eza 的输出

请运行 `nix build -L trydump` 或 `just idump`，
并留意是否有测试不再通过。
