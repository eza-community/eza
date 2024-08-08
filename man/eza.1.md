% eza(1) $version

<!-- This is the eza(1) man page, written in Markdown. -->
<!-- To generate the roff version, run `just man`, -->
<!-- and the man page will appear in the ‘target’ directory. -->


NAME
====

eza — a modern replacement for ls


SYNOPSIS
========

`eza [options] [files...]`

**eza** is a modern replacement for `ls`.
It uses colours for information by default, helping you distinguish between many types of files, such as whether you are the owner, or in the owning group.

It also has extra features not present in the original `ls`, such as viewing the Git status for a directory, or recursing into directories with a tree view.


EXAMPLES
========

`eza`
: Lists the contents of the current directory in a grid.

`eza --oneline --reverse --sort=size`
: Displays a list of files with the largest at the top.

`eza --long --header --inode --git`
: Displays a table of files with a header, showing each file’s metadata, inode, and Git status.

`eza --long --tree --level=3`
: Displays a tree of files, three levels deep, as well as each file’s metadata.


META OPTIONS
===============

`--help`
: Show list of command-line options.

`-v`, `--version`
: Show version of eza.


DISPLAY OPTIONS
===============

`-1`, `--oneline`
: Display one entry per line.

`-F`, `--classify=WHEN`
: Display file kind indicators next to file names.

Valid settings are ‘`always`’, ‘`automatic`’ (or ‘`auto`’ for short), and ‘`never`’.
The default value is ‘`automatic`’.

The default behavior (`automatic` or `auto`) will display file kind indicators only when the standard output is connected to a real terminal. If `eza` is ran while in a `tty`, or the output of `eza` is either redirected to a file or piped into another program, file kind indicators will not be used. Setting this option to ‘`always`’ causes `eza` to always display file kind indicators, while ‘`never`’ disables the use of file kind indicators.

`-G`, `--grid`
: Display entries as a grid (default).

`-l`, `--long`
: Display extended file metadata as a table.

`-R`, `--recurse`
: Recurse into directories.

`-T`, `--tree`
: Recurse into directories as a tree.

`-X`, `--dereference`
: Dereference symbolic links when displaying information.

`-x`, `--across`
: Sort the grid across, rather than downwards.

`--color=WHEN`, `--colour=WHEN`
: When to use terminal colours (using ANSI escape code to colorize the output).

Valid settings are ‘`always`’, ‘`automatic`’ (or ‘`auto`’ for short), and ‘`never`’.
The default value is ‘`automatic`’.

The default behavior (‘`automatic`’ or ‘`auto`’) is to colorize the output only when the standard output is connected to a real terminal. If the output of `eza` is redirected to a file or piped into another program, terminal colors will not be used. Setting this option to ‘`always`’ causes `eza` to always output terminal color, while ‘`never`’ disables the use of terminal color.

Manually setting this option overrides `NO_COLOR` environment.

`--color-scale`, `--colour-scale`
: highlight levels of `field` distinctly.
Use comma(,) separated list of all, age, size

`--color-scale-mode`, `--colour-scale-mode`
: Use gradient or fixed colors in `--color-scale`.

Valid options are `fixed` or `gradient`.
The default value is `gradient`.

`--icons=WHEN`
: Display icons next to file names.

Valid settings are ‘`always`’, ‘`automatic`’ (‘`auto`’ for short), and ‘`never`’.
The default value is ‘`automatic`’.

`automatic` or `auto` will display icons only when the standard output is connected to a real terminal. If `eza` is ran while in a `tty`, or the output of `eza` is either redirected to a file or piped into another program, icons will not be used. Setting this option to ‘`always`’ causes `eza` to always display icons, while ‘`never`’ disables the use of icons.

`--no-quotes`
: Don't quote file names with spaces.

`--hyperlink`
: Display entries as hyperlinks

`-w`, `--width=COLS`
: Set screen width in columns.

Valid options are `none`, `absolute` or `relative`.
The default value is `none`

`absolute` mode highlights based on file modification time relative to the past year.
`relative` mode highlights based on file modification time in relation to other files. `none` disables highlighting.

FILTERING AND SORTING OPTIONS
=============================

`-a`, `--all`
: Show hidden and “dot” files.
Use this twice to also show the ‘`.`’ and ‘`..`’ directories.

`-A`, `--almost-all`
: Equivalent to --all; included for compatibility with `ls -A`.

`-d`, `--list-dirs`
: List directories as regular files, rather than recursing and listing their contents.

`-L`, `--level=DEPTH`
: Limit the depth of recursion.

`-r`, `--reverse`
: Reverse the sort order.

`-s`, `--sort=SORT_FIELD`
: Which field to sort by.

Valid sort fields are ‘`name`’, ‘`Name`’, ‘`extension`’, ‘`Extension`’, ‘`size`’, ‘`modified`’, ‘`changed`’, ‘`accessed`’, ‘`created`’, ‘`inode`’, ‘`type`’, and ‘`none`’.

The `modified` sort field has the aliases ‘`date`’, ‘`time`’, and ‘`newest`’, and its reverse order has the aliases ‘`age`’ and ‘`oldest`’.

Sort fields starting with a capital letter will sort uppercase before lowercase: ‘A’ then ‘B’ then ‘a’ then ‘b’. Fields starting with a lowercase letter will mix them: ‘A’ then ‘a’ then ‘B’ then ‘b’.

`-I`, `--ignore-glob=GLOBS`
: Glob patterns, pipe-separated, of files to ignore.

`--git-ignore` [if eza was built with git support]
: Do not list files that are ignored by Git.

`--group-directories-first`
: List directories before other files.

`-D`, `--only-dirs`
: List only directories, not files.

`-f`, `--only-files`
: List only files, not directories.

`--show-symlinks`
: Explicitly show symbolic links (when used with `--only-files` | `--only-dirs`)

`--no-symlinks`
: Do not show symbolic links

LONG VIEW OPTIONS
=================

These options are available when running with `--long` (`-l`):

`-b`, `--binary`
: List file sizes with binary prefixes.

`-B`, `--bytes`
: List file sizes in bytes, without any prefixes.

`--changed`
: Use the changed timestamp field.

`-g`, `--group`
: List each file’s group.

`--smart-group`
: Only show group if it has a different name from owner

`-h WHEN`, `--header=WHEN`
: Add a header row to each column.

Valid settings are ‘`always`’, ‘`automatic`’ (‘`auto`’ for short), and ‘`never`’.
The default value is ‘`automatic`’.

`-H`, `--links`
: List each file’s number of hard links.

`-i`, `--inode`
: List each file’s inode number.

`-m`, `--modified`
: Use the modified timestamp field.

`-M`, `--mounts`
: Show mount details (Linux and Mac only)

`-n`, `--numeric`
: List numeric user and group IDs.

`-O`, `--flags`
: List file flags on Mac and BSD systems and file attributes on Windows systems.  By default, Windows attributes are displayed in a long form.  To display in attributes as single character set the environment variable `EZA_WINDOWS_ATTRIBUTES=short`.  On BSD systems see chflags(1) for a list of file flags and their meanings.

`-S`, `--blocksize`
: List each file’s size of allocated file system blocks.

`-t`, `--time=WORD`
: Which timestamp field to list.

: Valid timestamp fields are ‘`modified`’, ‘`changed`’, ‘`accessed`’, and ‘`created`’.

`--time-style=STYLE`
: How to format timestamps.

: Valid timestamp styles are ‘`default`’, ‘`iso`’, ‘`long-iso`’, ‘`full-iso`’, ‘`relative`’, or a custom style ‘`+<FORMAT>`’ (e.g., ‘`+%Y-%m-%d %H:%M`’ => ‘`2023-09-30 13:00`’).

`<FORMAT>` should be a chrono format string.  For details on the chrono format syntax, please read: https://docs.rs/chrono/latest/chrono/format/strftime/index.html .

Alternatively, `<FORMAT>` can be a two line string, the first line will be used for non-recent files and the second for recent files.  E.g., if `<FORMAT>` is "`%Y-%m-%d %H<newline>--%m-%d %H:%M`", non-recent files => "`2022-12-30 13`", recent files => "`--09-30 13:34`".

`--total-size`
: Show recursive directory size (unix only).

`-u`, `--accessed`
: Use the accessed timestamp field.

`-U`, `--created`
: Use the created timestamp field.

`--no-permissions`
: Suppress the permissions field.

`-o`, `--octal-permissions`
: List each file's permissions in octal format.

`--no-filesize`
: Suppress the file size field.

`--no-user`
: Suppress the user field.

`--no-time`
: Suppress the time field.

`--stdin`
: When you wish to pipe directories to eza/read from stdin. Separate one per line or define custom separation char in `EZA_STDIN_SEPARATOR` env variable.

`-@`, `--extended`
: List each file’s extended attributes and sizes.

`-Z`, `--context`
: List each file's security context.

`--git`  [if eza was built with git support]
: List each file’s Git status, if tracked.
This adds a two-character column indicating the staged and unstaged statuses respectively. The status character can be ‘`-`’ for not modified, ‘`M`’ for a modified file, ‘`N`’ for a new file, ‘`D`’ for deleted, ‘`R`’ for renamed, ‘`T`’ for type-change, ‘`I`’ for ignored, and ‘`U`’ for conflicted. Directories will be shown to have the status of their contents, which is how ‘deleted’ is possible if a directory contains a file that has a certain status, it will be shown to have that status.

`--git-repos` [if eza was built with git support]
: List each directory’s Git status, if tracked.
Symbols shown are `|`= clean, `+`= dirty, and `~`= for unknown.

`--git-repos-no-status` [if eza was built with git support]
: List if a directory is a Git repository, but not its status.
All Git repository directories will be shown as (themed) `-` without status indicated.


`--no-git`
: Don't show Git status (always overrides `--git`, `--git-repos`, `--git-repos-no-status`)


ENVIRONMENT VARIABLES
=====================

If an environment variable prefixed with `EZA_` is not set, for backward compatibility, it will default to its counterpart starting with `EXA_`.

eza responds to the following environment variables:

## `COLUMNS`

Overrides the width of the terminal, in characters, however, `-w` takes precedence.

For example, ‘`COLUMNS=80 eza`’ will show a grid view with a maximum width of 80 characters.

This option won’t do anything when eza’s output doesn’t wrap, such as when using the `--long` view.

## `EZA_STRICT`

Enables _strict mode_, which will make eza error when two command-line options are incompatible.

Usually, options can override each other going right-to-left on the command line, so that eza can be given aliases: creating an alias ‘`eza=eza --sort=ext`’ then running ‘`eza --sort=size`’ with that alias will run ‘`eza --sort=ext --sort=size`’, and the sorting specified by the user will override the sorting specified by the alias.

In strict mode, the two options will not co-operate, and eza will error.

This option is intended for use with automated scripts and other situations where you want to be certain you’re typing in the right command.

## `EZA_GRID_ROWS`

Limits the grid-details view (‘`eza --grid --long`’) so it’s only activated when at least the given number of rows of output would be generated.

With widescreen displays, it’s possible for the grid to look very wide and sparse, on just one or two lines with none of the columns lining up.
By specifying a minimum number of rows, you can only use the view if it’s going to be worth using.

## `EZA_ICON_SPACING`

Specifies the number of spaces to print between an icon (see the ‘`--icons`’ option) and its file name.

Different terminals display icons differently, as they usually take up more than one character width on screen, so there’s no “standard” number of spaces that eza can use to separate an icon from text. One space may place the icon too close to the text, and two spaces may place it too far away. So the choice is left up to the user to configure depending on their terminal emulator.

## `NO_COLOR`

Disables colours in the output (regardless of its value). Can be overridden by `--color` option.

See `https://no-color.org/` for details.

## `LS_COLORS`, `EZA_COLORS`

Specifies the colour scheme used to highlight files based on their name and kind, as well as highlighting metadata and parts of the UI.

For more information on the format of these environment variables, see the [eza_colors.5.md](eza_colors.5.md) manual page.

## `EZA_OVERRIDE_GIT`

Overrides any `--git` or `--git-repos` argument

## `EZA_MIN_LUMINANCE`
Specifies the minimum luminance to use when color-scale is active. It's value can be between -100 to 100.

## `EZA_ICONS_AUTO`

If set, automates the same behavior as using `--icons` or `--icons=auto`. Useful for if you always want to have icons enabled.

Any explicit use of the `--icons=WHEN` flag overrides this behavior. 

## `EZA_STDIN_SEPARATOR`

Specifies the separator to use when file names are piped from stdin. Defaults to newline.

EXIT STATUSES
=============

0
: If everything goes OK.

1
: If there was an I/O error during operation.

3
: If there was a problem with the command-line arguments.


AUTHOR
======

eza is maintained by Christina Sørensen and many other contributors.

**Source code:** `https://github.com/eza-community/eza` \
**Contributors:** `https://github.com/eza-community/eza/graphs/contributors`

Our infinite thanks to Benjamin ‘ogham’ Sago and all the other contributors of exa, from which eza was forked.

SEE ALSO
========

- [eza_colors.5.md](eza_colors.5.md)
- [eza_colors-explanation.5.md](eza_colors-explanation.5.md)
