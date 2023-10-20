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
- Music (mp3, m4a, ogg) is a deeper purple.
- Lossless music (flac, alac, wav) is deeper than *that* purple. In general, most media files are some shade of purple.
- Cryptographic files (asc, enc, p12) are a faint blue.
- Documents (pdf, doc, dvi) are a less faint blue.
- Compressed files (zip, tgz, Z) are red.
- Temporary files (tmp, swp, ~) are grey.
- Compiled files (class, o, pyc) are yellow. A file is also counted as compiled if it uses a common extension and is
in the same directory as one of its source files: styles.css will count as compiled when next to styles.less or styles.sass, and scripts.js when next to scripts.ts or scripts.coffee.
- Source files (cpp, js, java) are bright yellow.


## See also

- [eza.1.md](eza.1.md)
- [eza_colors.5.md](eza_colors.5.md)
