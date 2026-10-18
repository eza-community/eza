% eza_colors-explanation(5) $version

<!-- This is the eza_colors-explanation(5) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->
<!-- 本文件为 eza_colors-explanation(5) 手册页原文的简体中文译本。 -->

> **语言**：[English](https://github.com/eza-community/eza/blob/main/man/eza_colors-explanation.5.md) | 简体中文（本文档）
>
> 本译文中的 YAML 字段名、环境变量名、颜色取值与命令行片段均保持原文，以便直接复用。

# Name（名称）

eza_colors-explanation —— 关于自定义 eza 颜色的更多细节

# Eza 颜色说明

eza 内置了一套自己的文件扩展名映射，覆盖面很广，涵盖文档、归档包、媒体文件与临时文件等常见扩展名。
环境变量中的任何映射都会覆盖这套默认映射：以 `LS_COLORS="*.zip=32"` 运行 eza 会把 zip 文件变为绿色，而其他压缩文件的颜色则保持不变。

你也可以在 `EZA_COLORS` 的开头加入一个
`reset` 条目，从而完全禁用这套内置映射。
因此，设置 `EZA_COLORS="reset:*.txt=31"` 只会让文本文件高亮；
而设置 `EZA_COLORS="reset"` 则不会高亮任何内容。

## 示例

- 禁用“当前用户”高亮：`EZA_COLORS="uu=0:gu=0"`
- 将日期列变为绿色：`EZA_COLORS="da=32"`
- 高亮 Vagrantfile：`EZA_COLORS="Vagrantfile=1;4;33"`
- 覆盖现有的 zip 文件颜色：`EZA_COLORS="*.zip=38;5;125"`
- Markdown 文件呈某种绿色，日志文件呈某种灰色：
`EZA_COLORS="*.md=38;5;121:*.log=38;5;248"`

## BUILT\-IN EXTENSIONS（内置扩展名映射）

- eza 现已支持高亮（bright）颜色！正如大多数现代 256 色终端所支持的那样，在 `#EZA_COLORS` 环境变量中选择自定义颜色时，你也可以选用 `bright` 系列的颜色代码。

- 构建文件（Makefile、Cargo.toml、package.json）为黄色并带下划线。
- 图片（png、jpeg、gif）为紫色。
- 视频（mp4、ogv、m2ts）为略偏紫的紫色。
- 音乐（mp3、m4a、ogg）为淡蓝色。
- 无损音乐（flac、alac、wav）为较深一些的淡蓝色。
- 加密文件（asc、enc、p12）为亮绿色。
- 文档（pdf、doc、dvi）为更淡的绿色。
- 压缩文件（zip、tgz、Z）为红色。
- 临时文件（tmp、swp、~）为变暗的默认前景色。
- 已编译文件（class、o、pyc）为黄色。此外，如果某个文件使用了常见扩展名，且与其某个源文件位于同一目录下，也会被算作已编译文件：styles.css 在紧邻 styles.less 或 styles.sass 时会算作已编译文件，scripts.js 在紧邻 scripts.ts 或 scripts.coffee 时同理。
- 源文件（cpp、js、java）为亮黄色。


## 主题配置文件

现在，你可以在一个 `theme.yml` 文件中用方便的语法定义你的样式，从而指定上述选项及更多内容。

通过设置 `EZA_CONFIG_DIR`，可以指定你希望 eza 在哪个目录中查找 `theme.yml` 文件；
若不设置，eza 会在 `$XDG_CONFIG_HOME/eza/theme.yml` 中查找。


以下是可用选项：

LIST OF THEME OPTIONS（主题选项清单）
=====================

```yaml
filekinds:
  normal
  directory
  symlink
  pipe
  block_device
  char_device
  socket
  special
  executable
  mount_point

perms:
  user_read
  user_write
  user_execute_file
  user_execute_other
  group_read
  group_write
  group_execute
  other_read
  other_write
  other_execute
  special_user_file
  special_other
  attribute

size:
  major
  minor
  number_byte
  number_kilo
  number_mega
  number_giga
  number_huge
  unit_byte
  unit_kilo
  unit_mega
  unit_giga
  unit_huge

users:
  user_you
  user_root
  user_other
  group_yours
  group_other
  group_root

links:
  normal
  multi_link_file

git:
  new
  modified
  deleted
  renamed
  ignored
  conflicted

git_repo:
  branch_main
  branch_other
  git_clean
  git_dirty

security_context:
  none:
  selinux:
    colon
    user
    role
    typ
    range

file_type:
  image
  video
  music
  crypto
  document
  compressed
  temp
  compiled
  build
  source

punctuation:

date:

inode:

blocks:

header:

octal:

flags:

control_char:

broken_symlink:

broken_path_overlay:

```

上述每个字段 / 子字段之下，都可以定义以下样式属性

```yaml
    foreground: Blue
    background: null
    is_bold: false
    is_dimmed: false
    is_italic: false
    is_underline: false
    is_blink: false
    is_reverse: false
    is_hidden: false
    is_strikethrough: true
    prefix_with_reset: false
```

示例：

```yaml

file_type:
  image:
    foreground: Blue
    is_italic: true
date:
  foreground: White

security_context:
  selinux:
    role:
      is_hidden: true
```

现在还可以在 `filenames` 与 `extensions` 字段中自定义图标

```yaml

filenames:
  # 仅改变图标字形
  Cargo.toml: {icon: {glyph: 🦀}}
  Cargo.lock: {icon: {glyph: 🦀}}

extensions:
  rs: {  filename: {foreground: Red}, icon: {glyph: 🦀}}

```

**注意：**

并非所有字形都支持更改颜色。

如果你的主题未能正常工作，请仔细检查配置文件中的语法，因为
语法问题可能导致多个属性无法生效。

无论你指定哪个目录，该文件都必须命名为 `theme.yml`。


## See also（参见）

- [**eza**(1)](eza.1.zh-CN.md)
- [**eza_colors**(5)](eza_colors.5.zh-CN.md)
