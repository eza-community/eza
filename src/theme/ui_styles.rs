use ansiterm::Style;

use crate::theme::lsc::Pair;

#[rustfmt::skip]
#[derive(Debug, Default, PartialEq)]
pub struct UiStyles {
    pub colourful: bool,

    pub filekinds:        FileKinds,
    pub perms:            Permissions,
    pub size:             Size,
    pub users:            Users,
    pub links:            Links,
    pub git:              Git,
    pub git_repo:         GitRepo,
    pub security_context: SecurityContext,
    pub file_type:        FileType,

    pub punctuation:  Style,          // xx
    pub date:         Style,          // da
    pub inode:        Style,          // in
    pub blocks:       Style,          // bl
    pub header:       Style,          // hd
    pub octal:        Style,          // oc
    pub flags:        Style,          // ff

    pub symlink_path:         Style,  // lp
    pub control_char:         Style,  // cc
    pub broken_symlink:       Style,  // or
    pub broken_path_overlay:  Style,  // bO
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FileKinds {
    pub normal: Style,        // fi
    pub directory: Style,     // di
    pub symlink: Style,       // ln
    pub pipe: Style,          // pi
    pub block_device: Style,  // bd
    pub char_device: Style,   // cd
    pub socket: Style,        // so
    pub special: Style,       // sp
    pub executable: Style,    // ex
    pub mount_point: Style,   // mp
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Permissions {
    pub user_read:          Style,  // ur
    pub user_write:         Style,  // uw
    pub user_execute_file:  Style,  // ux
    pub user_execute_other: Style,  // ue

    pub group_read:    Style,       // gr
    pub group_write:   Style,       // gw
    pub group_execute: Style,       // gx

    pub other_read:    Style,       // tr
    pub other_write:   Style,       // tw
    pub other_execute: Style,       // tx

    pub special_user_file: Style,   // su
    pub special_other:     Style,   // sf

    pub attribute: Style,           // xa
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub major: Style,        // df
    pub minor: Style,        // ds

    pub number_byte: Style,  // sn nb
    pub number_kilo: Style,  // sn nk
    pub number_mega: Style,  // sn nm
    pub number_giga: Style,  // sn ng
    pub number_huge: Style,  // sn nt

    pub unit_byte: Style,    // sb ub
    pub unit_kilo: Style,    // sb uk
    pub unit_mega: Style,    // sb um
    pub unit_giga: Style,    // sb ug
    pub unit_huge: Style,    // sb ut
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Users {
    pub user_you: Style,           // uu
    pub user_root: Style,          // uR
    pub user_other: Style,         // un
    pub group_yours: Style,        // gu
    pub group_other: Style,        // gn
    pub group_root: Style,         // gR
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Links {
    pub normal: Style,           // lc
    pub multi_link_file: Style,  // lm
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Git {
    pub new: Style,         // ga
    pub modified: Style,    // gm
    pub deleted: Style,     // gd
    pub renamed: Style,     // gv
    pub typechange: Style,  // gt
    pub ignored: Style,     // gi
    pub conflicted: Style,  // gc
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GitRepo {
    pub branch_main: Style,  //Gm
    pub branch_other: Style, //Go
    pub git_clean: Style,    //Gc
    pub git_dirty: Style,    //Gd
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SELinuxContext {
    pub colon: Style,
    pub user: Style,  // Su
    pub role: Style,  // Sr
    pub typ: Style,   // St
    pub range: Style, // Sl
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecurityContext {
    pub none:    Style, // Sn
    pub selinux: SELinuxContext,
}

/// Drawing styles based on the type of file (video, image, compressed, etc)
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FileType {
    pub image: Style,       // im - image file
    pub video: Style,       // vi - video file
    pub music: Style,       // mu - lossy music
    pub lossless: Style,    // lo - lossless music
    pub crypto: Style,      // cr - related to cryptography
    pub document: Style,    // do - document file
    pub compressed: Style,  // co - compressed file
    pub temp: Style,        // tm - temporary file
    pub compiled: Style,    // cm - compilation artifact
    pub build: Style,       // bu - file that is used to build a project
    pub source: Style,      // sc - source code
}

impl UiStyles {
    pub fn plain() -> Self {
        Self::default()
    }
}

impl UiStyles {
    /// Sets a value on this set of colours using one of the keys understood
    /// by the `LS_COLORS` environment variable. Invalid keys set nothing, but
    /// return false.
    pub fn set_ls(&mut self, pair: &Pair<'_>) -> bool {
        #[rustfmt::skip]
        match pair.key {
            "di" => self.filekinds.directory    = pair.to_style(),  // DIR
            "ex" => self.filekinds.executable   = pair.to_style(),  // EXEC
            "fi" => self.filekinds.normal       = pair.to_style(),  // FILE
            "pi" => self.filekinds.pipe         = pair.to_style(),  // FIFO
            "so" => self.filekinds.socket       = pair.to_style(),  // SOCK
            "bd" => self.filekinds.block_device = pair.to_style(),  // BLK
            "cd" => self.filekinds.char_device  = pair.to_style(),  // CHR
            "ln" => self.filekinds.symlink      = pair.to_style(),  // LINK
            "or" => self.broken_symlink         = pair.to_style(),  // ORPHAN
             _   => return false,
             // Codes we don’t do anything with:
             // MULTIHARDLINK, DOOR, SETUID, SETGID, CAPABILITY,
             // STICKY_OTHER_WRITABLE, OTHER_WRITABLE, STICKY, MISSING
        };
        true
    }

    /// Sets a value on this set of colours using one of the keys understood
    /// by the `EZA_COLORS` environment variable. Invalid keys set nothing,
    /// but return false. This doesn’t take the `LS_COLORS` keys into account,
    /// so `set_ls` should have been run first.
    pub fn set_exa(&mut self, pair: &Pair<'_>) -> bool {
        #[rustfmt::skip]
        match pair.key {
            "ur" => self.perms.user_read                = pair.to_style(),
            "uw" => self.perms.user_write               = pair.to_style(),
            "ux" => self.perms.user_execute_file        = pair.to_style(),
            "ue" => self.perms.user_execute_other       = pair.to_style(),
            "gr" => self.perms.group_read               = pair.to_style(),
            "gw" => self.perms.group_write              = pair.to_style(),
            "gx" => self.perms.group_execute            = pair.to_style(),
            "tr" => self.perms.other_read               = pair.to_style(),
            "tw" => self.perms.other_write              = pair.to_style(),
            "tx" => self.perms.other_execute            = pair.to_style(),
            "su" => self.perms.special_user_file        = pair.to_style(),
            "sf" => self.perms.special_other            = pair.to_style(),
            "xa" => self.perms.attribute                = pair.to_style(),

            "sn" => self.set_number_style(pair.to_style()),
            "sb" => self.set_unit_style(pair.to_style()),
            "nb" => self.size.number_byte               = pair.to_style(),
            "nk" => self.size.number_kilo               = pair.to_style(),
            "nm" => self.size.number_mega               = pair.to_style(),
            "ng" => self.size.number_giga               = pair.to_style(),
            "nt" => self.size.number_huge               = pair.to_style(),
            "ub" => self.size.unit_byte                 = pair.to_style(),
            "uk" => self.size.unit_kilo                 = pair.to_style(),
            "um" => self.size.unit_mega                 = pair.to_style(),
            "ug" => self.size.unit_giga                 = pair.to_style(),
            "ut" => self.size.unit_huge                 = pair.to_style(),
            "df" => self.size.major                     = pair.to_style(),
            "ds" => self.size.minor                     = pair.to_style(),

            "uu" => self.users.user_you                 = pair.to_style(),
            "un" => self.users.user_other               = pair.to_style(),
            "uR" => self.users.user_root                = pair.to_style(),
            "gu" => self.users.group_yours              = pair.to_style(),
            "gn" => self.users.group_other              = pair.to_style(),
            "gR" => self.users.group_root               = pair.to_style(),

            "lc" => self.links.normal                   = pair.to_style(),
            "lm" => self.links.multi_link_file          = pair.to_style(),

            "ga" => self.git.new                        = pair.to_style(),
            "gm" => self.git.modified                   = pair.to_style(),
            "gd" => self.git.deleted                    = pair.to_style(),
            "gv" => self.git.renamed                    = pair.to_style(),
            "gt" => self.git.typechange                 = pair.to_style(),
            "gi" => self.git.ignored                    = pair.to_style(),
            "gc" => self.git.conflicted                 = pair.to_style(),

            "Gm" => self.git_repo.branch_main           = pair.to_style(),
            "Go" => self.git_repo.branch_other          = pair.to_style(),
            "Gc" => self.git_repo.git_clean             = pair.to_style(),
            "Gd" => self.git_repo.git_dirty             = pair.to_style(),

            "xx" => self.punctuation                    = pair.to_style(),
            "da" => self.date                           = pair.to_style(),
            "in" => self.inode                          = pair.to_style(),
            "bl" => self.blocks                         = pair.to_style(),
            "hd" => self.header                         = pair.to_style(),
            "oc" => self.octal                          = pair.to_style(),
            "ff" => self.flags                          = pair.to_style(),
            "lp" => self.symlink_path                   = pair.to_style(),
            "cc" => self.control_char                   = pair.to_style(),
            "bO" => self.broken_path_overlay            = pair.to_style(),

            "mp" => self.filekinds.mount_point          = pair.to_style(),
            "sp" => self.filekinds.special              = pair.to_style(),  // Catch-all for unrecognized file kind

            "im" => self.file_type.image                = pair.to_style(),
            "vi" => self.file_type.video                = pair.to_style(),
            "mu" => self.file_type.music                = pair.to_style(),
            "lo" => self.file_type.lossless             = pair.to_style(),
            "cr" => self.file_type.crypto               = pair.to_style(),
            "do" => self.file_type.document             = pair.to_style(),
            "co" => self.file_type.compressed           = pair.to_style(),
            "tm" => self.file_type.temp                 = pair.to_style(),
            "cm" => self.file_type.compiled             = pair.to_style(),
            "bu" => self.file_type.build                = pair.to_style(),
            "sc" => self.file_type.source               = pair.to_style(),

            "Sn" => self.security_context.none          = pair.to_style(),
            "Su" => self.security_context.selinux.user  = pair.to_style(),
            "Sr" => self.security_context.selinux.role  = pair.to_style(),
            "St" => self.security_context.selinux.typ   = pair.to_style(),
            "Sl" => self.security_context.selinux.range = pair.to_style(),

             _   => return false,
        };

        true
    }

    pub fn set_number_style(&mut self, style: Style) {
        self.size.number_byte = style;
        self.size.number_kilo = style;
        self.size.number_mega = style;
        self.size.number_giga = style;
        self.size.number_huge = style;
    }

    pub fn set_unit_style(&mut self, style: Style) {
        self.size.unit_byte = style;
        self.size.unit_kilo = style;
        self.size.unit_mega = style;
        self.size.unit_giga = style;
        self.size.unit_huge = style;
    }
}
