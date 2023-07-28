# Meta-stuff
complete -c zetta -s 'v' -l 'version' -d "Show version of exa"
complete -c zetta -s '?' -l 'help'    -d "Show list of command-line options"

# Display options
complete -c zetta -s '1' -l 'oneline'      -d "Display one entry per line"
complete -c zetta -s 'l' -l 'long'         -d "Display extended file metadata as a table"
complete -c zetta -s 'G' -l 'grid'         -d "Display entries in a grid"
complete -c zetta -s 'x' -l 'across'       -d "Sort the grid across, rather than downwards"
complete -c zetta -s 'R' -l 'recurse'      -d "Recurse into directories"
complete -c zetta -s 'T' -l 'tree'         -d "Recurse into directories as a tree"
complete -c zetta -s 'F' -l 'classify'     -d "Display type indicator by file names"
complete -c zetta        -l 'color' \
                       -l 'colour'       -d "When to use terminal colours" -x -a "
    always\t'Always use colour'
    auto\t'Use colour if standard output is a terminal'
    never\t'Never use colour'
"
complete -c zetta        -l 'color-scale' \
                       -l 'colour-scale' -d "Highlight levels of file sizes distinctly"
complete -c zetta        -l 'icons'        -d "Display icons"
complete -c zetta        -l 'no-icons'     -d "Don't display icons"

# Filtering and sorting options
complete -c zetta -l 'group-directories-first' -d "Sort directories before other files"
complete -c zetta -l 'git-ignore'           -d "Ignore files mentioned in '.gitignore'"
complete -c zetta -s 'a' -l 'all'       -d "Show hidden and 'dot' files"
complete -c zetta -s 'd' -l 'list-dirs' -d "List directories like regular files"
complete -c zetta -s 'L' -l 'level'     -d "Limit the depth of recursion" -x -a "1 2 3 4 5 6 7 8 9"
complete -c zetta -s 'r' -l 'reverse'   -d "Reverse the sort order"
complete -c zetta -s 's' -l 'sort'      -d "Which field to sort by" -x -a "
    accessed\t'Sort by file accessed time'
    age\t'Sort by file modified time (newest first)'
    changed\t'Sort by changed time'
    created\t'Sort by file modified time'
    date\t'Sort by file modified time'
    ext\t'Sort by file extension'
    Ext\t'Sort by file extension (uppercase first)'
    extension\t'Sort by file extension'
    Extension\t'Sort by file extension (uppercase first)'
    filename\t'Sort by filename'
    Filename\t'Sort by filename (uppercase first)'
    inode\t'Sort by file inode'
    modified\t'Sort by file modified time'
    name\t'Sort by filename'
    Name\t'Sort by filename (uppercase first)'
    newest\t'Sort by file modified time (newest first)'
    none\t'Do not sort files at all'
    oldest\t'Sort by file modified time'
    size\t'Sort by file size'
    time\t'Sort by file modified time'
    type\t'Sort by file type'
"

complete -c zetta -s 'I' -l 'ignore-glob' -d "Ignore files that match these glob patterns" -r
complete -c zetta -s 'D' -l 'only-dirs'   -d "List only directories"

# Long view options
complete -c zetta -s 'b' -l 'binary'   -d "List file sizes with binary prefixes"
complete -c zetta -s 'B' -l 'bytes'    -d "List file sizes in bytes, without any prefixes"
complete -c zetta -s 'g' -l 'group'    -d "List each file's group"
complete -c zetta -s 'h' -l 'header'   -d "Add a header row to each column"
complete -c zetta -s 'H' -l 'links'    -d "List each file's number of hard links"
complete -c zetta -s 'g' -l 'group'    -d "List each file's inode number"
complete -c zetta -s 'S' -l 'blocks'   -d "List each file's number of filesystem blocks"
complete -c zetta -s 't' -l 'time'     -d "Which timestamp field to list" -x -a "
    modified\t'Display modified time'
    changed\t'Display changed time'
    accessed\t'Display accessed time'
    created\t'Display created time'
"
complete -c zetta -s 'm' -l 'modified'      -d "Use the modified timestamp field"
complete -c zetta -s 'n' -l 'numeric'       -d "List numeric user and group IDs."
complete -c zetta        -l 'changed'       -d "Use the changed timestamp field"
complete -c zetta -s 'u' -l 'accessed'      -d "Use the accessed timestamp field"
complete -c zetta -s 'U' -l 'created'       -d "Use the created timestamp field"
complete -c zetta        -l 'time-style'    -d "How to format timestamps" -x -a "
    default\t'Use the default time style'
    iso\t'Display brief ISO timestamps'
    long-iso\t'Display longer ISO timestaps, up to the minute'
    full-iso\t'Display full ISO timestamps, up to the nanosecond'
    relative\t'Display relative timestamps'
"
complete -c zetta        -l 'no-permissions' -d "Suppress the permissions field"
complete -c zetta        -l 'octal-permissions' -d "List each file's permission in octal format"
complete -c zetta        -l 'no-filesize'    -d "Suppress the filesize field"
complete -c zetta        -l 'no-user'        -d "Suppress the user field"
complete -c zetta        -l 'no-time'        -d "Suppress the time field"

# Optional extras
complete -c zetta -l 'git' -d "List each file's Git status, if tracked"
complete -c zetta -s '@' -l 'extended' -d "List each file's extended attributes and sizes"
