use crate::output::color_scale::ColorScaleOptions;
use crate::theme::lsc::Pair;
use libc::SYS_socketpair;
use nu_ansi_term::Color::{self, *};
use nu_ansi_term::Style;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UiStyles {
    pub colourful: Option<bool>,

    pub filekinds:        Option<FileKinds>,
    pub perms:            Option<Permissions>,
    pub size:             Option<Size>,
    pub users:            Option<Users>,
    pub links:           Option<Links>,
    pub git:              Option<Git>,
    pub git_repo:         Option<GitRepo>,
    pub security_context: Option<SecurityContext>,
    pub file_type:        Option<FileType>,

    pub punctuation:  Option<Style>,          // xx
    pub date:         Option<Style>,          // da
    pub inode:        Option<Style>,          // in
    pub blocks:       Option<Style>,          // bl
    pub header:       Option<Style>,          // hd
    pub octal:        Option<Style>,          // oc
    pub flags:        Option<Style>,          // ff

    pub icon:         Option<Style>,  // ic
    pub symlink_path:         Option<Style>,  // lp
    pub control_char:         Option<Style>,  // cc
    pub broken_symlink:       Option<Style>,  // or
    pub broken_path_overlay:  Option<Style>,  // bO
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileKinds {
    pub normal: Option<Style>,        // fi
    pub directory: Option<Style>,     // di
    pub symlink: Option<Style>,       // ln
    pub pipe: Option<Style>,          // pi
    pub block_device: Option<Style>,  // bd
    pub char_device: Option<Style>,   // cd
    pub socket: Option<Style>,        // so
    pub special: Option<Style>,       // sp
    pub executable: Option<Style>,    // ex
    pub mount_point: Option<Style>,   // mp
}

impl Default for FileKinds {
    fn default() -> Self {
        Self {
            normal: Some(Style::default()),
            directory: Some(Blue.bold()),
            symlink: Some(Cyan.normal()),
            pipe: Some(Yellow.normal()),
            block_device: Some(Yellow.bold()),
            char_device: Some(Yellow.bold()),
            socket: Some(Red.bold()),
            special: Some(Yellow.normal()),
            executable: Some(Green.bold()),
            mount_point: Some(Blue.bold().underline()),
        }
    }
}

impl FileKinds {
    pub fn colourful(scale: ColorScaleOptions) -> Self {
        Self {
            normal: Some(Style::default()),
            directory: Some(scale.blue.bold()),
            symlink: Some(scale.cyan.normal()),
            pipe: Some(scale.yellow.normal()),
            block_device: Some(scale.yellow.bold()),
            char_device: Some(scale.yellow.bold()),
            socket: Some(scale.red.bold()),
            special: Some(scale.yellow.normal()),
            executable: Some(scale.green.bold()),
            mount_point: Some(scale.blue.bold().underline()),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub user_read:         Option<Style>,  // ur
    pub user_write:         Option<Style>,  // uw
    pub user_execute_file:  Option<Style>,  // ux
    pub user_execute_other: Option<Style>,  // ue

    pub group_read:    Option<Style>,       // gr
    pub group_write:   Option<Style>,       // gw
    pub group_execute: Option<Style>,       // gx

    pub other_read:    Option<Style>,       // tr
    pub other_write:   Option<Style>,       // tw
    pub other_execute: Option<Style>,       // tx

    pub special_user_file: Option<Style>,   // su
    pub special_other:     Option<Style>,   // sf

    pub attribute: Option<Style>,           // xa
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub major: Option<Style>,        // df
    pub minor: Option<Style>,        // ds

    pub number_byte: Option<Style>,  // sn nb
    pub number_kilo: Option<Style>,  // sn nk
    pub number_mega: Option<Style>,  // sn nm
    pub number_giga: Option<Style>,  // sn ng
    pub number_huge: Option<Style>,  // sn nt

    pub unit_byte: Option<Style>,    // sb ub
    pub unit_kilo: Option<Style>,    // sb uk
    pub unit_mega: Option<Style>,    // sb um
    pub unit_giga: Option<Style>,    // sb ug
    pub unit_huge: Option<Style>,    // sb ut
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Users {
    pub user_you: Option<Style>,           // uu
    pub user_root: Option<Style>,          // uR
    pub user_other: Option<Style>,         // un
    pub group_yours: Option<Style>,        // gu
    pub group_other: Option<Style>,        // gn
    pub group_root: Option<Style>,         // gR
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Links {
    pub normal: Option<Style>,           // lc
    pub multi_link_file: Option<Style>,  // lm
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Git {
    pub new: Option<Style>,         // ga
    pub modified: Option<Style>,    // gm
    pub deleted: Option<Style>,     // gd
    pub renamed: Option<Style>,     // gv
    pub typechange: Option<Style>,  // gt
    pub ignored: Option<Style>,     // gi
    pub conflicted: Option<Style>,  // gc
}

impl Default for Git {
    fn default() -> Self {
        Git {
            new: Some(Green.normal()),
            modified: Some(Blue.normal()),
            deleted: Some(Red.normal()),
            renamed: Some(Yellow.normal()),
            typechange: Some(Purple.normal()),
            ignored: Some(Style::default().dimmed()),
            conflicted: Some(Red.normal()),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepo {
    pub branch_main: Option<Style>,  //Gm
    pub branch_other: Option<Style>, //Go
    pub git_clean: Option<Style>,    //Gc
    pub git_dirty: Option<Style>,    //Gd
}

impl Default for GitRepo {
    fn default() -> Self {
        Self {
            branch_main: Some(Green.normal()),
            branch_other: Some(Yellow.normal()),
            git_clean: Some(Green.normal()),
            git_dirty: Some(Yellow.bold()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SELinuxContext {
    pub colon: Option<Style>,
    pub user: Option<Style>,  // Su
    pub role: Option<Style>,  // Sr
    pub typ: Option<Style>,   // St
    pub range: Option<Style>, // Sl
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecurityContext {
    pub none:    Option<Style>, // Sn
    pub selinux: Option<SELinuxContext>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        SecurityContext {
            none: Some(Style::default()),
            selinux: Some(SELinuxContext {
                colon: Some(Style::default().dimmed()),
                user: Some(Blue.normal()),
                role: Some(Green.normal()),
                typ: Some(Yellow.normal()),
                range: Some(Cyan.normal()),
            }),
        }
    }
}

/// Drawing styles based on the type of file (video, image, compressed, etc)
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileType {
    pub image: Option<Style>,       // im - image file
    pub video: Option<Style>,       // vi - video file
    pub music: Option<Style>,       // mu - lossy music
    pub lossless: Option<Style>,    // lo - lossless music
    pub crypto: Option<Style>,      // cr - related to cryptography
    pub document: Option<Style>,    // do - document file
    pub compressed: Option<Style>,  // co - compressed file
    pub temp: Option<Style>,        // tm - temporary file
    pub compiled: Option<Style>,    // cm - compilation artifact
    pub build: Option<Style>,       // bu - file that is used to build a project
    pub source: Option<Style>,      // sc - source code
}

impl UiStyles {
    pub fn plain() -> Self {
        Self::default()
    }

    pub fn from_yaml(file: Option<&str>) -> Self {
        if let Some(file) = file {
            let file = std::fs::File::open(file);
            if let Err(e) = file {
                eprintln!("Could not open theme file: {}", e);
                return Self::default();
            }
            let file = file.expect("Could not open theme file");
            let theme: UiStyles = serde_yaml::from_reader(file).unwrap_or_else(|e| {
                eprintln!("Could not parse theme file: {}", e);
                Self::default()
            });
            theme
        } else {
            Self::default()
        }
    }
}

impl UiStyles {
    /// Sets a value on this set of colours using one of the keys understood
    /// by the `LS_COLORS` environment variable. Invalid keys set nothing, but
    /// return false.
    pub fn set_ls(&mut self, pair: &Pair<'_>) -> bool {
        #[rustfmt::skip]
        match pair.key {
            "di" => self.filekinds.directory    = Some(pair.to_style()),  // DIR
            "ex" => self.filekinds.executable   = Some(pair.to_style()),  // EXEC
            "fi" => self.filekinds.normal       = Some(pair.to_style()),  // FILE
            "pi" => self.filekinds.pipe         = Some(pair.to_style()),  // FIFO
            "so" => self.filekinds.socket       = Some(pair.to_style()),  // SOCK
            "bd" => self.filekinds.block_device = Some(pair.to_style()),  // BLK
            "cd" => self.filekinds.char_device  = Some(pair.to_style()),  // CHR
            "ln" => self.filekinds.symlink      = Some(pair.to_style()),  // LINK
            "or" => self.broken_symlink         = Some(pair.to_style()),  // ORPHAN
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
            "ur" => self.perms.user_read                = Some(pair.to_style()),
            "uw" => self.perms.user_write               = Some(pair.to_style()),
            "ux" => self.perms.user_execute_file        = Some(pair.to_style()),
            "ue" => self.perms.user_execute_other       = Some(pair.to_style()),
            "gr" => self.perms.group_read               = Some(pair.to_style()),
            "gw" => self.perms.group_write              = Some(pair.to_style()),
            "gx" => self.perms.group_execute            = Some(pair.to_style()),
            "tr" => self.perms.other_read               = Some(pair.to_style()),
            "tw" => self.perms.other_write              = Some(pair.to_style()),
            "tx" => self.perms.other_execute            = Some(pair.to_style()),
            "su" => self.perms.special_user_file        = Some(pair.to_style()),
            "sf" => self.perms.special_other            = Some(pair.to_style()),
            "xa" => self.perms.attribute                = Some(pair.to_style()),

            "sn" => self.set_number_style(pair.to_style()),
            "sb" => self.set_unit_style(pair.to_style()),
            "nb" => self.size.number_byte               = Some(pair.to_style()),
            "nk" => self.size.number_kilo               = Some(pair.to_style()),
            "nm" => self.size.number_mega               = Some(pair.to_style()),
            "ng" => self.size.number_giga               = Some(pair.to_style()),
            "nt" => self.size.number_huge               = Some(pair.to_style()),
            "ub" => self.size.unit_byte                 = Some(pair.to_style()),
            "uk" => self.size.unit_kilo                 = Some(pair.to_style()),
            "um" => self.size.unit_mega                 = Some(pair.to_style()),
            "ug" => self.size.unit_giga                 = Some(pair.to_style()),
            "ut" => self.size.unit_huge                 = Some(pair.to_style()),
            "df" => self.size.major                     = Some(pair.to_style()),
            "ds" => self.size.minor                     = Some(pair.to_style()),

            "uu" => self.users.user_you                 = Some(pair.to_style()),
            "un" => self.users.user_other               = Some(pair.to_style()),
            "uR" => self.users.user_root                = Some(pair.to_style()),
            "gu" => self.users.group_yours              = Some(pair.to_style()),
            "gn" => self.users.group_other              = Some(pair.to_style()),
            "gR" => self.users.group_root               = Some(pair.to_style()),

            "lc" => self.links.normal                   = Some(pair.to_style()),
            "lm" => self.links.multi_link_file          = Some(pair.to_style()),

            "ga" => self.git.new                        = Some(pair.to_style()),
            "gm" => self.git.modified                   = Some(pair.to_style()),
            "gd" => self.git.deleted                    = Some(pair.to_style()),
            "gv" => self.git.renamed                    = Some(pair.to_style()),
            "gt" => self.git.typechange                 = Some(pair.to_style()),
            "gi" => self.git.ignored                    = Some(pair.to_style()),
            "gc" => self.git.conflicted                 = Some(pair.to_style()),

            "Gm" => self.git_repo.branch_main           = Some(pair.to_style()),
            "Go" => self.git_repo.branch_other          = Some(pair.to_style()),
            "Gc" => self.git_repo.git_clean             = Some(pair.to_style()),
            "Gd" => self.git_repo.git_dirty             = Some(pair.to_style()),

            "ic" => self.icon                           = Some(pair.to_style()),

            "xx" => self.punctuation                    = Some(pair.to_style()),
            "da" => self.date                           = Some(pair.to_style()),
            "in" => self.inode                          = Some(pair.to_style()),
            "bl" => self.blocks                         = Some(pair.to_style()),
            "hd" => self.header                         = Some(pair.to_style()),
            "oc" => self.octal                          = Some(pair.to_style()),
            "ff" => self.flags                          = Some(pair.to_style()),
            "lp" => self.symlink_path                   = Some(pair.to_style()),
            "cc" => self.control_char                   = Some(pair.to_style()),
            "bO" => self.broken_path_overlay            = Some(pair.to_style()),
            "mp" => self.filekinds.mount_point          = Some(pair.to_style()),
            "sp" => self.filekinds.special              = Some(pair.to_style()),  // Catch-all for unrecognized file kind

            "im" => self.file_type.image                = Some(pair.to_style()),
            "vi" => self.file_type.video                = Some(pair.to_style()),
            "mu" => self.file_type.music                = Some(pair.to_style()),
            "lo" => self.file_type.lossless             = Some(pair.to_style()),
            "cr" => self.file_type.crypto               = Some(pair.to_style()),
            "do" => self.file_type.document             = Some(pair.to_style()),
            "co" => self.file_type.compressed           = Some(pair.to_style()),
            "tm" => self.file_type.temp                 = Some(pair.to_style()),
            "cm" => self.file_type.compiled             = Some(pair.to_style()),
            "bu" => self.file_type.build                = Some(pair.to_style()),
            "sc" => self.file_type.source               = Some(pair.to_style()),

            "Sn" => self.security_context.none          = Some(pair.to_style()),
            "Su" => self.security_context.selinux.user  = Some(pair.to_style()),
            "Sr" => self.security_context.selinux.role  = Some(pair.to_style()),
            "St" => self.security_context.selinux.typ   = Some(pair.to_style()),
            "Sl" => self.security_context.selinux.range = Some(pair.to_style()),

             _   => return false,
        };

        true
    }

    pub fn set_number_style(&mut self, style: Style) {
        self.size.number_byte = Some(style);
        self.size.number_kilo = Some(style);
        self.size.number_mega = Some(style);
        self.size.number_giga = Some(style);
        self.size.number_huge = Some(style);
    }

    pub fn set_unit_style(&mut self, style: Style) {
        self.size.unit_byte = Some(style);
        self.size.unit_kilo = Some(style);
        self.size.unit_mega = Some(style);
        self.size.unit_giga = Some(style);
        self.size.unit_huge = Some(style);
    }
}
