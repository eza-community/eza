% eza(1) $version

<!-- This is the eza(1) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->
<!-- 本文件为 eza(1) 手册页原文的简体中文译本。 -->

> **语言**：[English](https://github.com/eza-community/eza/blob/main/man/eza.1.md) | 简体中文（本文档）
>
> 本译文中的命令、选项名、环境变量名、文件路径与技术标识符均保持原文，以确保可直接对照使用。


NAME（名称）
====

eza — 现代化的 `ls` 替代品


SYNOPSIS（用法概要）
========

`eza [options] [files...]`

**eza** 是 `ls` 的现代化替代品。
它默认使用颜色来呈现信息，帮助你区分多种类型的文件，例如区分你是文件的所有者，还是属于该文件所属的用户组。

它还提供了原版 `ls` 所没有的额外特性，例如查看某个目录的 Git 状态，或以树状视图递归展开目录。


EXAMPLES（示例）
========

`eza`
: 以网格方式列出当前目录的内容。

`eza --oneline --reverse --sort=size`
: 列出文件清单，并将最大的文件排在顶部。

`eza --long --header --inode --git`
: 以带表头的表格形式列出文件，显示每个文件的元数据、inode 编号与 Git 状态。

`eza --long --tree --level=3`
: 以三层深度的目录树形式列出文件，同时显示每个文件的元数据。


META OPTIONS（元选项）
===============

`--help`
: 显示命令行选项列表。

`-v`, `--version`
: 显示 eza 的版本。

DISPLAY OPTIONS（显示选项）
===============

`-1`, `--oneline`
: 每行显示一个条目。

`--absolute=WHEN`
: 以绝对路径显示条目。

有效取值为 '`on`'、'`follow`' 与 '`off`'。
若不指定取值，默认为 '`on`'。

'`on`'：为所有条目显示绝对路径。
'`follow`'：显示绝对路径，并将符号链接解析为其指向的目标。
'`off`'：显示相对路径（默认行为）。

`-F`, `--classify=WHEN`
: 在文件名旁显示文件类型标识符。

有效取值为 ‘`always`’、‘`automatic`’（简写为 ‘`auto`’）与 ‘`never`’。
若不指定取值，默认为 ‘`automatic`’。

`automatic` 或 `auto` 仅在标准输出连接到真实终端时才显示文件类型标识符。如果 `eza` 在 `tty` 中运行，或者 `eza` 的输出被重定向到文件、或通过管道传给另一个程序，就不会使用文件类型标识符。将该选项设为 ‘`always`’ 会让 `eza` 始终显示文件类型标识符，而 ‘`never`’ 则会禁用文件类型标识符。

`-G`, `--grid`
: 以网格方式显示条目（默认）。

`-l`, `--long`
: 以表格形式显示扩展的文件元数据。

`-R`, `--recurse`
: 递归进入子目录。

`-T`, `--tree`
: 以树状结构递归进入子目录。

`--code[=MODE]`
: 不列出文件，而是按编程语言输出代码行数统计摘要，思路类似 `tokei` 与 `cloc` 这类工具。

: 指定的路径（若未指定则为当前目录）会被递归遍历；当存在 Git 仓库时会遵循其 `.gitignore`。每种被识别出来的语言都会报告其文件数、行数、代码行数、注释行数与空行数，并附上一根用于可视化该语言代码占比的条形图。有效模式为 ‘`lines`’、‘`percent`’ 与 ‘`both`’（默认值）。

`--follow-symlinks`
: 深入指向目录的符号链接内部。

`-X`, `--dereference`
: 显示文件信息时解引用（dereference）符号链接。

`-x`, `--across`
: 网格按横向排序，而非纵向。

`--color=WHEN`, `--colour=WHEN`
: 何时使用终端颜色（使用 ANSI 转义序列为输出着色）。

有效取值为 ‘`always`’、‘`automatic`’（简写为 ‘`auto`’）与 ‘`never`’。
若不指定取值，默认为 ‘`automatic`’。

默认行为（‘`automatic`’ 或 ‘`auto`’）是仅在标准输出连接到真实终端时才为输出着色。如果 `eza` 的输出被重定向到文件或通过管道传给另一个程序，就不会使用终端颜色。将该选项设为 ‘`always`’ 会让 `eza` 始终输出终端颜色，而 ‘`never`’ 则会禁用终端颜色。

手动设置该选项会覆盖 `NO_COLOR` 环境变量。

`--color-scale`, `--colour-scale`
: 以差异化的方式高亮 `field` 字段的取值。
使用逗号（`,`）分隔的 `all`、`age`、`size` 列表。

`--color-scale-mode=MODE`, `--colour-scale-mode=MODE`
: 在 `--color-scale` 中使用渐变色或固定色。

有效选项为 `fixed` 或 `gradient`。
若不指定取值，默认为 `gradient`。

`--icons=WHEN`
: 在文件名旁显示图标。

有效取值为 ‘`always`’、‘`automatic`’（简写为 ‘`auto`’）与 ‘`never`’。
若不指定取值，默认为 ‘`automatic`’。

`automatic` 或 `auto` 仅在标准输出连接到真实终端时才显示图标。如果 `eza` 在 `tty` 中运行，或者 `eza` 的输出被重定向到文件、或通过管道传给另一个程序，就不会使用图标。将该选项设为 ‘`always`’ 会让 `eza` 始终显示图标，而 ‘`never`’ 则会禁用图标。

`--no-quotes`
: 不要为含有空格的文件名加引号。

`--short-nix`
: 缩写文件名与路径中的 Nix store 哈希。

: 以 Nix store 哈希开头的路径片段 —— 即恰好 32 个 Nix base32 字母表的字符后跟一个连字符，例如 `vlkia5wk0svsikwv50554mh06iayg2m2-source.drv` —— 会将其哈希缩写为前 8 个字符加省略号，并以暗色（dim）显示，使文件名更醒目：`vlkia5wk…-source.drv`。该规则适用于列出的文件名、符号链接目标以及绝对路径。

`--hyperlink=WHEN`
: 以超链接形式显示条目。

有效取值为 ‘`always`’、‘`automatic`’（简写为 ‘`auto`’）与 ‘`never`’。
若不指定取值，默认为 ‘`automatic`’。

`automatic` 或 `auto` 仅在标准输出连接到真实终端时才显示超链接。如果 `eza` 在 `tty` 中运行，或者 `eza` 的输出被重定向到文件、或通过管道传给另一个程序，就不会使用超链接。将该选项设为 ‘`always`’ 会让 `eza` 始终显示超链接，而 ‘`never`’ 则会禁用超链接。

`-w`, `--width=COLS`
: 以列为单位设置屏幕宽度。

FILTERING AND SORTING OPTIONS（过滤与排序选项）
=============================

`-a`, `--all`
: 显示隐藏文件与“点文件”（dot file）。
使用两次还会显示 ‘`.`’ 与 ‘`..`’ 目录。

`-A`, `--almost-all`
: 等价于 `--all`；为兼容 `ls -A` 而提供。

`-d`, `--treat-dirs-as-files`
: 该标志继承自 `ls`，用于改变 `eza` 处理目录参数的方式。

: 默认情况下 `eza` 会进入目录并列出其内容；使用该标志后，`eza` 会将目录当作普通文件处理，只列出该目录条目本身的信息。

: 当你想查看目录本身的元数据（例如权限、大小、修改时间）而不是其内容时，这个选项会很有用。

: 如果你只是想仅列出目录而不列文件，可以考虑改用 `--only-dirs`（`-D`）选项。

`-L`, `--level=DEPTH`
: 限制递归深度。

`-r`, `--reverse`
: 反转排序顺序。

`-s`, `--sort=SORT_FIELD`
: 按哪个字段排序。

有效的排序字段为 ‘`name`’、‘`Name`’、‘`extension`’、‘`Extension`’、‘`size`’、‘`modified`’、‘`changed`’、‘`accessed`’、‘`created`’、‘`inode`’、‘`type`’ 与 ‘`none`’。

`modified` 排序字段具有别名 ‘`date`’、‘`time`’ 与 ‘`newest`’；其反向顺序具有别名 ‘`age`’ 与 ‘`oldest`’。

以大写字母开头的排序字段会将大写字母排在小写字母之前：‘A’、‘B’、‘a’、‘b’。以小写字母开头的字段会将大小写混合排列：‘A’、‘a’、‘B’、‘b’。

`-I`, `--ignore-glob=GLOBS`
: 需要忽略的文件的 glob 模式，以竖线分隔。

`--git-ignore` [若构建 eza 时启用了 git 支持]
: 不列出被 Git 忽略的文件。

`--group-directories-first`
: 将目录排在其他文件之前。

`--group-directories-last`
: 将目录排在其他文件之后。

`-D`, `--only-dirs`
: 只列出目录，不列出文件。

`-f`, `--only-files`
: 只列出文件，不列出目录。

`--show-symlinks`
: 显式显示符号链接（与 `--only-files` | `--only-dirs` 配合使用时）

`--no-symlinks`
: 不显示符号链接

LONG VIEW OPTIONS（长视图选项）
=================

以下选项需配合 `--long`（`-l`）使用：

`-b`, `--binary`
: 以二进制前缀列出文件大小。

`-B`, `--bytes`
: 以字节为单位列出文件大小，不带任何前缀。

`--changed`
: 使用变更时间（changed）时间戳字段。

`-g`, `--group`
: 列出每个文件的所属组。

`--smart-group`
: 仅当组名与所有者名不同时才显示组。

`-h`, `--header`
: 为每列添加表头行。

`-H`, `--links`
: 列出每个文件的硬链接数。

`-i`, `--inode`
: 列出每个文件的 inode 编号。

`--loc[=MODE]`
: 在长视图中添加“语言”列与“代码行数”列。

: 只有属于已知编程语言的普通文件才会被统计；统计过程能识别注释，因此代码行数这一列不含注释行与空行。

: 有效模式为 ‘`lines`’（代码行数）、‘`percent`’（每个文件占整棵树代码的份额）与 ‘`both`’（默认值）。在 `percent` 与 `both` 模式下，分母是被递归遍历的整棵树（若存在 Git 仓库则为其范围内）的全部代码行数。

`-m`, `--modified`
: 使用修改时间（modified）时间戳字段。

`-M`, `--mounts`
: 显示挂载详情（仅支持 Linux 与 Mac）

`-n`, `--numeric`
: 以数字形式列出用户 ID 与组 ID。

`-O`, `--flags`
: 在 Mac 与 BSD 系统上列出文件标志（file flags），在 Windows 系统上列出文件属性。默认情况下，Windows 属性以完整形式显示。若要以单字符形式显示属性，请设置环境变量 `EZA_WINDOWS_ATTRIBUTES=short`。在 BSD 系统上，可参阅 chflags(1) 了解文件标志及其含义的列表。

`-S`, `--blocksize`
: 列出每个文件已分配的文件系统块大小。

`-t`, `--time=WORD`
: 列出哪个时间戳字段。

: 有效的时间戳字段为 ‘`modified`’、‘`changed`’、‘`accessed`’ 与 ‘`created`’。

`--time-style=STYLE`
: 如何格式化时间戳。

: 有效的时间戳样式为 ‘`default`’、‘`iso`’、‘`long-iso`’、‘`full-iso`’、‘`relative`’，或自定义样式 ‘`+<FORMAT>`’（例如 ‘`+%Y-%m-%d %H:%M`’ 对应 ‘`2023-09-30 13:00`’）。

`<FORMAT>` 应为 chrono 格式字符串。有关 chrono 格式语法的详细信息，请参阅：https://docs.rs/chrono/latest/chrono/format/strftime/index.html 。

此外，`<FORMAT>` 也可以是双行字符串，第一行用于非近期文件，第二行用于近期文件。例如，若 `<FORMAT>` 为 “`%Y-%m-%d %H<newline>--%m-%d %H:%M`”，则非近期文件显示为 “`2022-12-30 13`”，近期文件显示为 “`--09-30 13:34`”。

`--total-size`
: 显示目录的递归总大小（仅 Unix）。

`-u`, `--accessed`
: 使用访问时间（accessed）时间戳字段。

`-U`, `--created`
: 使用创建时间（created）时间戳字段。

`--no-permissions`
: 隐藏权限字段。

`-o`, `--octal-permissions`
: 以八进制格式列出每个文件的权限。

`--no-filesize`
: 隐藏文件大小字段。

`--no-user`
: 隐藏所有者字段。

`--no-time`
: 隐藏时间字段。

`--stdin`
: 当你希望通过管道把目录传给 eza / 从标准输入读取时使用。各条目每行一个，或可在 `EZA_STDIN_SEPARATOR` 环境变量中自定义分隔符。

`-@`, `--extended`
: 列出每个文件的扩展属性及其大小。

`-Z`, `--context`
: 列出每个文件的安全上下文。

`--git`  [若构建 eza 时启用了 git 支持]
: 若文件被 Git 跟踪，则列出其 Git 状态。
这会新增一个两字符的列，分别表示暂存区（staged）与工作区（unstaged）的状态。状态字符可以是 ‘`-`’ 表示未修改，‘`M`’ 表示文件已被修改，‘`N`’ 表示新文件，‘`D`’ 表示已删除，‘`R`’ 表示已重命名，‘`T`’ 表示类型变更，‘`I`’ 表示被忽略，‘`U`’ 表示存在冲突。目录会显示为其内容的状态，因此“已删除”这种情况也可能出现在目录上：如果某个目录中包含具有某种状态的文件，该目录就会显示为相应的状态。

`--git-repos` [若构建 eza 时启用了 git 支持]
: 若目录被 Git 跟踪，则列出其 Git 状态。
所显示的符号为：`|` 表示干净，`+` 表示有改动，`~` 表示未知。

`--git-repos-no-status` [若构建 eza 时启用了 git 支持]
: 列出某个目录是否为 Git 仓库，但不显示其状态。
所有 Git 仓库目录都会显示为（按主题着色的）`-`，且不指示具体状态。


`--no-git`
: 不显示 Git 状态（始终覆盖 `--git`、`--git-repos`、`--git-repos-no-status`）


ENVIRONMENT VARIABLES（环境变量）
=====================

如果某个以 `EZA_` 为前缀的环境变量未设置，出于向后兼容考虑，它会回退到以 `EXA_` 开头的对应变量。

eza 会响应以下环境变量：

## `COLUMNS`

覆盖终端宽度（以字符为单位），不过 `-w` 优先级更高。

例如，‘`COLUMNS=80 eza`’ 会显示最大宽度为 80 个字符的网格视图。

当 eza 的输出不会换行时（例如使用 `--long` 视图时），该选项不产生作用。

## `EZA_STRICT`

启用_严格模式_（strict mode）：当两个命令行选项互不兼容时，eza 会报错。

通常情况下，命令行上的选项可以从右到左相互覆盖，这样就能为 eza 设置别名：创建别名 ‘`eza=eza --sort=ext`’ 后，再通过该别名运行 ‘`eza --sort=size`’，实际会执行 ‘`eza --sort=ext --sort=size`’，用户指定的排序方式会覆盖别名中的排序方式。

而在严格模式下，这两个选项不会相互协作，eza 会报错。

该选项适用于自动化脚本，以及其他你希望确保自己输入了正确命令的场景。

## `EZA_GRID_ROWS`

限制 grid-details 视图（‘`eza --grid --long`’）：只有当输出至少达到指定的行数时，才会启用该视图。

在宽屏显示器上，网格可能会显得非常宽且稀疏，只占一两行，而且各列根本无法对齐。
通过指定最小行数，你可以确保只在值得使用该视图时才使用它。

## `EZA_ICON_SPACING`

指定在图标（参见 ‘`--icons`’ 选项）与其文件名之间打印的空格数量。

不同的终端显示图标的方式不同，因为图标在屏幕上通常占据超过一个字符的宽度，所以并不存在一个可供 eza 使用的“标准”空格数来分隔图标与文本。一个空格可能让图标贴得离文本太近，两个空格又可能离得太远。因此这一选择留给用户，由其根据自身所用的终端模拟器进行配置。

## `NO_COLOR`

禁用输出中的颜色（无论其取值为何）。可被 `--color` 选项覆盖。

详见 `https://no-color.org/`。

## `LS_COLORS`、`EZA_COLORS`

指定用于根据文件名称与类型高亮显示文件的配色方案，同时也用于高亮元数据与部分界面元素。

有关这些环境变量格式的更多信息，请参阅 [eza_colors.5.md](eza_colors.5.md) 手册页（中译：[eza_colors.5.zh-CN.md](eza_colors.5.zh-CN.md)）。

## `EZA_OVERRIDE_GIT`

覆盖任何 `--git` 或 `--git-repos` 参数。

## `EZA_MIN_LUMINANCE`
当 color-scale 生效时，指定所使用的最小亮度（luminance）。其取值范围为 -100 到 100。

## `EZA_ICONS_AUTO`

若设置该变量，会自动实现与使用 `--icons` 或 `--icons=auto` 相同的行为。适用于你希望始终启用图标的场景。

对 `--icons=WHEN` 标志的任何显式使用都会覆盖该行为。

## `EZA_STDIN_SEPARATOR`

指定从标准输入以管道传入文件名时所使用的分隔符。默认为换行符。

## `EZA_CONFIG_DIR`

指定 eza 查找其配置文件与主题文件的目录。默认为 `$XDG_CONFIG_HOME/eza`；若未设置 `XDG_CONFIG_HOME`，则为 `$HOME/.config/eza`。

EXIT STATUSES（退出状态码）
=============

0
: 一切正常。

1
: 操作过程中发生了 I/O 错误。

3
: 命令行参数存在问题。

13
: 访问某路径时权限被拒绝。


AUTHOR（作者）
======

eza 由 Christina Sørensen 与众多其他贡献者维护。

**源代码：** `https://github.com/eza-community/eza` \
**贡献者：** `https://github.com/eza-community/eza/graphs/contributors`

衷心感谢 Benjamin ‘ogham’ Sago 以及 exa 的所有其他贡献者 —— eza 正是从 exa 派生而来的。

SEE ALSO（参见）
========

- [**eza_colors**(5)](eza_colors.5.zh-CN.md)
- [**eza_colors-explanation**(5)](eza_colors-explanation.5.zh-CN.md)
