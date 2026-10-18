% eza_colors(5) $version

<!-- This is the eza_colors(5) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->
<!-- 本文件为 eza_colors(5) 手册页原文的简体中文译本。 -->

> **语言**：[English](https://github.com/eza-community/eza/blob/main/man/eza_colors.5.md) | 简体中文（本文档）
>
> 本译文中的环境变量名、双字母代码、ANSI 取值、glob 表达式与技术标识符均保持原文，以便直接使用。


NAME（名称）
====

eza_colors —— 自定义 eza 的文件与界面颜色


SYNOPSIS（用法概要）
========

`EZA_COLORS` 环境变量可用于自定义 `eza` 用来高亮文件名、文件元数据以及部分界面元素的颜色。

你可以使用 `dircolors` 程序生成一段脚本，根据输入文件设置该变量；如果你不介意编辑长串文本，也可以直接手动输入。这些变量具有以下结构：

- 若干由 ‘`=`’ 分隔的键值对条目，例如 ‘`*.txt=32`’。
- 多个 ANSI 格式码之间以 ‘`;`’ 分隔，例如 ‘`*.txt=32;1;4`’。
- 最后，多组条目之间以 ‘`:`’ 分隔，例如 ‘`*.txt=32:*.mp3=1;35`’。

键值对的键可以是双字母代码，也可以是一个文件 glob；凡不是有效代码的键都将被视为 glob，即使它恰好是两个字母。

出于向后兼容考虑，若未设置 `EZA_COLORS`，则会检查 `EXA_COLORS` 环境变量。


EXAMPLES（示例）
========

`EZA_COLORS="uu=0:gu=0"`
: 禁用“当前用户”高亮

`EZA_COLORS="da=32"`
: 将日期列变为绿色

`EZA_COLORS="Vagrantfile=1;4;33"`
: 高亮 Vagrantfile

`EZA_COLORS="*.zip=38;5;125"`
: 覆盖现有的 zip 文件颜色

`EZA_COLORS="*.md=38;5;121:*.log=38;5;248"`
: Markdown 文件呈某种绿色，日志文件呈某种灰色


LIST OF CODES（代码清单）
=============

`LS_COLORS` 可使用以下十个代码：

`di`
: 目录

`ex`
: 可执行文件

`fi`
: 普通文件

`pi`
: 命名管道

`so`
: 套接字

`bd`
: 块设备

`cd`
: 字符设备

`ln`
: 符号链接

`or`
: 指向目标不存在的符号链接


`EZA_COLORS` 可使用更多代码：

`oc`
: 以八进制显示的权限

`ur`
: 属主读（user-read）权限位

`uw`
: 属主写（user-write）权限位

`ux`
: 普通文件的属主执行（user-execute）权限位

`ue`
: 其他文件类型的属主执行权限位

`gr`
: 属组读（group-read）权限位

`gw`
: 属组写（group-write）权限位

`gx`
: 属组执行（group-execute）权限位

`tr`
: 其他用户读（others-read）权限位

`tw`
: 其他用户写（others-write）权限位

`tx`
: 其他用户执行（others-execute）权限位

`su`
: 文件的 setuid、setgid 与粘滞（sticky）权限位

`sf`
: 其他文件类型的 setuid、setgid 与粘滞权限位

`xa`
: 扩展属性指示符

`sn`
: 文件大小的数字部分（会同时设置 `nb`、`nk`、`nm`、`ng` 与 `nt`）

`nb`
: 文件大小小于 1 KB/KiB 时的数字部分

`nk`
: 文件大小介于 1 KB/KiB 与 1 MB/MiB 之间时的数字部分

`nm`
: 文件大小介于 1 MB/MiB 与 1 GB/GiB 之间时的数字部分

`ng`
: 文件大小介于 1 GB/GiB 与 1 TB/TiB 之间时的数字部分

`nt`
: 文件大小为 1 TB/TiB 及以上时的数字部分

`sb`
: 文件大小的单位部分（会同时设置 `ub`、`uk`、`um`、`ug` 与 `ut`）

`ub`
: 文件大小小于 1 KB/KiB 时的单位部分

`uk`
: 文件大小介于 1 KB/KiB 与 1 MB/MiB 之间时的单位部分

`um`
: 文件大小介于 1 MB/MiB 与 1 GB/GiB 之间时的单位部分

`ug`
: 文件大小介于 1 GB/GiB 与 1 TB/TiB 之间时的单位部分

`ut`
: 文件大小为 1 TB/TiB 及以上时的单位部分

`df`
: 设备的主设备号（major ID）

`ds`
: 设备的次设备号（minor ID）

`uu`
: 属于你本人的用户

`uR`
: root 用户

`un`
: 其他用户

`gu`
: 你所属的组

`gR`
: 与 root 相关的组

`gn`
: 你不属于的组

`lc`
: 硬链接数

`lm`
: 至少有两个硬链接的普通文件的硬链接数

`ga`
: Git 中的“新增”标志

`gm`
: Git 中的“已修改”标志

`gd`
: Git 中的“已删除”标志

`gv`
: Git 中的“已重命名”标志

`gt`
: Git 中的“元数据已修改”标志

`gi`
: Git 中的“已忽略”标志

`gc`
: Git 中的“存在冲突”标志

`Gm`
: 仓库的主分支

`Go`
: 仓库的其他分支

`Gc`
: 干净的仓库分支

`Gd`
: 有改动的仓库分支

`xx`
: “标点符号”，包括许多背景界面元素

`da`
: 文件的日期

`in`
: 文件的 inode 编号

`bl`
: 文件占用的块数

`hd`
: 表格的表头行

`lp`
: 符号链接的路径

`cc`
: 文件名中的转义字符

`bO`
: 失效符号链接路径的叠加样式

`sp`
: 特殊文件（既不是文件、目录、挂载点、可执行文件、管道、套接字、块设备、字符设备，也不是链接）

`mp`
: 挂载点

`im`
: 作为图片的普通文件

`vi`
: 作为视频的普通文件

`mu`
: 作为有损音乐的普通文件

`lo`
: 作为无损音乐的普通文件

`cr`
: 与加密相关的普通文件（例如密钥或证书）

`do`
: 作为文档的普通文件（例如办公套件文档或 PDF）

`co`
: 经过压缩的普通文件

`tm`
: 作为临时文件的普通文件（例如文本编辑器的备份文件）

`cm`
: 作为编译产物的普通文件（例如 Java class 文件）

`bu`
: 用于构建项目的普通文件（例如 Makefile）

`sc`
: 作为源代码的普通文件

`ic`
: 图标（该项可选；若未设置，图标颜色与文件名颜色一致）

`Sn`
: 文件上没有安全上下文

`Su`
: SELinux 用户

`Sr`
: SELinux 角色

`St`
: SELinux 类型

`Sl`
: SELinux 级别

`ff`
: BSD 文件标志

`EXA_COLORS` 中的取值会覆盖 `LS_COLORS` 中给出的取值，因此你无需为了让 `LS_COLORS` 支持专有扩展而重写它。


LIST OF STYLES（样式清单）
==============

与某些版本的 `ls` 不同，这里给出的 ANSI 取值必须是有效的颜色代码：eza 不会原样输出所给定的任意字符。

eza 接受的代码如下：

`1`
: 加粗

`2`
: 变暗（dim）

`3`
: 斜体

`4`
: 下划线

`31`
: 红色文字

`32`
: 绿色文字

`33`
: 黄色文字

`34`
: 蓝色文字

`35`
: 紫色文字

`36`
: 青色文字

`37`
: 白色文字

`90`
: 深灰色文字

`91`
: 亮红色文字

`92`
: 亮绿色文字

`93`
: 亮黄色文字

`94`
: 亮蓝色文字

`95`
: 亮紫色文字

`96`
: 亮青色文字

`97`
: 亮白色文字

`38;5;nnn`
: 取 0 到 255 之间的某种颜色（替换其中的 `nnn` 部分）

许多终端会把加粗的文字当作另一种颜色来渲染，或者至少提供这样的选项。

eza 内置了一套自己的文件扩展名映射，覆盖面很广，涵盖文档、归档包、媒体文件与临时文件等常见扩展名。
环境变量中的任何映射都会覆盖这套默认映射：以 `LS_COLORS="*.zip=32"` 运行 eza 会把 zip 文件变为绿色，而其他压缩文件的颜色则保持不变。

你也可以在 `EZA_COLORS` 的开头加入一个 `reset` 条目，从而完全禁用这套内置映射。
因此，设置 `EZA_COLORS="reset:*.txt=31"` 只会让文本文件高亮；而设置 `EZA_COLORS="reset"` 则不会高亮任何内容。


AUTHOR（作者）
======

eza 由 Christina Sørensen 与众多其他贡献者维护。

**源代码：** `https://github.com/eza-community/eza` \
**贡献者：** `https://github.com/eza-community/eza/graphs/contributors`

衷心感谢 Benjamin ‘ogham’ Sago 以及 exa 的所有其他贡献者 —— eza 正是从 exa 派生而来的。


SEE ALSO（参见）
========

- [**eza**(1)](eza.1.zh-CN.md)
- [**eza_colors-explanation**(5)](eza_colors-explanation.5.zh-CN.md)
