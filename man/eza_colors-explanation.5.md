% eza_colors-explanation(5) $version

<!-- This is the eza_colors-explanation(5) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->

# Name

eza_colors-explanation — more details on customizing eza colors

# Eza Color Explanation

eza provides its own built\-in set of file extension mappings that cover a large range of common file extensions, including documents, archives, media, and temporary files. 
Any mappings in the environment variables will override this default set: running eza with `LS_COLORS="*.zip=32"` will turn zip files green but leave the colours of other compressed files alone.

You can also disable this built\-in set entirely by including a
`reset` entry at the beginning of `EZA_COLORS`.
So setting `EZA_COLORS="reset:*.txt=31"` will highlight only text
files; setting `EZA_COLORS="reset"` will highlight nothing.

## Examples

- Disable the "current user" highlighting: `EZA_COLORS="uu=0:gu=0"`
- Turn the date column green: `EZA_COLORS="da=32"`
- Highlight Vagrantfiles: `EZA_COLORS="Vagrantfile=1;4;33"`
- Override the existing zip colour: `EZA_COLORS="*.zip=38;5;125"`
- Markdown files a shade of green, log files a shade of grey:
`EZA_COLORS="*.md=38;5;121:*.log=38;5;248"`

## BUILT\-IN EXTENSIONS

- eza now supports bright colours! As supported by most modern 256\-colour terminals, you can now choose from `bright` colour codes when selecting your custom colours in your `#EZA_COLORS` environment variable.

- Build (Makefile, Cargo.toml, package.json) are yellow and underlined.
- Images (png, jpeg, gif) are purple.
- Videos (mp4, ogv, m2ts) are a slightly purpler purple.
- Music (mp3, m4a, ogg) is a faint blue.
- Lossless music (flac, alac, wav) is a less faint blue.
- Cryptographic files (asc, enc, p12) are bright green.
- Documents (pdf, doc, dvi) are a fainter green.
- Compressed files (zip, tgz, Z) are red.
- Temporary files (tmp, swp, ~) are dimmed default foreground color.
- Compiled files (class, o, pyc) are yellow. A file is also counted as compiled if it uses a common extension and is
in the same directory as one of its source files: styles.css will count as compiled when next to styles.less or styles.sass, and scripts.js when next to scripts.ts or scripts.coffee.
- Source files (cpp, js, java) are bright yellow.


## Theme Configuration file

Now you can specify these options and more in a `theme.yml` file with convenient syntax for defining your styles.

Set `EZA_CONFIG_DIR` to specify which directory you would like eza to look for your `theme.yml` file,
otherwise eza will look for `$XDG_CONFIG_HOME/eza/theme.yml`.


These are the available options:

LIST OF THEME OPTIONS
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
  user_executable_file
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

Each of those fields/sub fields can have the following styling properties defined beneath it

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

Example:

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

Icons can now be customized as well in the `filenames` and `extensions` fields

```yaml

filenames:
  # Just change the icon glyph
  Cargo.toml: {icon: {glyph: 🦀}}
  Cargo.lock: {icon: {glyph: 🦀}}

extensions:
  rs: {  filename: {foreground: Red}, icon: {glyph: 🦀}}

```

**NOTES:** 

Not all glyphs support changing colors.

If your theme is not working properly, double check the syntax in the config file, as
a syntax issue can cause multiple properties to not be applied.

You must name the file `theme.yml`, no matter the directory you specify.


## See also

- [**eza**(1)](eza.1.md)
- [**eza_colors**(5)](eza_colors.5.md)
