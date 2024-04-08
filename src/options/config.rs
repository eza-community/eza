use crate::theme::ThemeFileType as FileType;
use crate::theme::*;
use nu_ansi_term::{Color, Style};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ThemeConfig {
    // This is rather bare for now, will be expanded with config file
    location: ConfigLoc,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ConfigLoc {
    #[default]
    Default, // $XDG_CONFIG_HOME/eza/config|theme.yml
    Env(PathBuf), // $EZA_CONFIG_DIR
}

trait FromOverride<T>: Sized {
    fn from(value: T) -> Self;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct StyleOverride {
    /// The style's foreground color, if it has one.
    pub foreground: Option<Color>,

    /// The style's background color, if it has one.
    pub background: Option<Color>,

    /// Whether this style is bold.
    pub is_bold: Option<bool>,

    /// Whether this style is dimmed.
    pub is_dimmed: Option<bool>,

    /// Whether this style is italic.
    pub is_italic: Option<bool>,

    /// Whether this style is underlined.
    pub is_underline: Option<bool>,

    /// Whether this style is blinking.
    pub is_blink: Option<bool>,

    /// Whether this style has reverse colors.
    pub is_reverse: Option<bool>,

    /// Whether this style is hidden.
    pub is_hidden: Option<bool>,

    /// Whether this style is struckthrough.
    pub is_strikethrough: Option<bool>,

    /// Wether this style is always displayed starting with a reset code to clear any remaining style artifacts
    pub prefix_with_reset: Option<bool>,
}

impl FromOverride<StyleOverride> for Style {
    fn from(value: StyleOverride) -> Self {
        let mut style = Style::default();
        if value.foreground.is_some() {
            style.foreground = value.foreground;
        }
        if value.background.is_some() {
            style.background = value.background;
        }
        if let Some(bold) = value.is_bold {
            style.is_bold = bold;
        }
        if let Some(dimmed) = value.is_dimmed {
            style.is_dimmed = dimmed;
        }
        if let Some(italic) = value.is_italic {
            style.is_italic = italic;
        }
        if let Some(underline) = value.is_underline {
            style.is_underline = underline;
        }
        if let Some(blink) = value.is_blink {
            style.is_blink = blink;
        }
        if let Some(reverse) = value.is_reverse {
            style.is_reverse = reverse;
        }
        if let Some(hidden) = value.is_hidden {
            style.is_hidden = hidden;
        }
        if let Some(strikethrough) = value.is_strikethrough {
            style.is_strikethrough = strikethrough;
        }
        if let Some(reset) = value.prefix_with_reset {
            style.prefix_with_reset = reset;
        }
        style
    }
}

impl FromOverride<Option<StyleOverride>> for Option<Style> {
    fn from(value: Option<StyleOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct IconStyleOverride {
    pub icon: Option<char>,
    pub style: Option<StyleOverride>,
}

impl FromOverride<IconStyleOverride> for IconStyle {
    fn from(value: IconStyleOverride) -> Self {
        IconStyle {
            icon: value.icon,
            style: FromOverride::from(value.style),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IconStylesOverride {
    pub filenames: Option<HashMap<String, IconStyleOverride>>,
    pub extensions: Option<HashMap<String, IconStyleOverride>>,
}

impl FromOverride<IconStylesOverride> for IconStyles {
    fn from(value: IconStylesOverride) -> Self {
        IconStyles {
            filenames: value.filenames.map(|map| {
                map.into_iter()
                    .map(|(k, v)| (k, FromOverride::from(v)))
                    .collect()
            }),
            extensions: value.extensions.map(|map| {
                map.into_iter()
                    .map(|(k, v)| (k, FromOverride::from(v)))
                    .collect()
            }),
        }
    }
}

impl FromOverride<Option<IconStylesOverride>> for Option<IconStyles> {
    fn from(value: Option<IconStylesOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Eq, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileKindsOverride {
    pub normal: Option<StyleOverride>,        // fi
    pub directory: Option<StyleOverride>,     // di
    pub symlink: Option<StyleOverride>,       // ln
    pub pipe: Option<StyleOverride>,          // pi
    pub block_device: Option<StyleOverride>,  // bd
    pub char_device: Option<StyleOverride>,   // cd
    pub socket: Option<StyleOverride>,        // so
    pub special: Option<StyleOverride>,       // sp
    pub executable: Option<StyleOverride>,    // ex
    pub mount_point: Option<StyleOverride>,   // mp
}

impl FromOverride<FileKindsOverride> for FileKinds {
    fn from(value: FileKindsOverride) -> Self {
        FileKinds {
            normal: FromOverride::from(value.normal),
            directory: FromOverride::from(value.directory),
            symlink: FromOverride::from(value.symlink),
            pipe: FromOverride::from(value.pipe),
            block_device: FromOverride::from(value.block_device),
            char_device: FromOverride::from(value.char_device),
            socket: FromOverride::from(value.socket),
            special: FromOverride::from(value.special),
            executable: FromOverride::from(value.executable),
            mount_point: FromOverride::from(value.mount_point),
        }
    }
}

impl FromOverride<Option<FileKindsOverride>> for Option<FileKinds> {
    fn from(value: Option<FileKindsOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy,Eq, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PermissionsOverride {
    pub user_read:         Option<StyleOverride>,  // ur
    pub user_write:         Option<StyleOverride>,  // uw
    pub user_execute_file:  Option<StyleOverride>,  // ux
    pub user_execute_other: Option<StyleOverride>,  // ue

    pub group_read:    Option<StyleOverride>,       // gr
    pub group_write:   Option<StyleOverride>,       // gw
    pub group_execute: Option<StyleOverride>,       // gx

    pub other_read:    Option<StyleOverride>,       // tr
    pub other_write:   Option<StyleOverride>,       // tw
    pub other_execute: Option<StyleOverride>,       // tx

    pub special_user_file: Option<StyleOverride>,   // su
    pub special_other:     Option<StyleOverride>,   // sf

    pub attribute: Option<StyleOverride>,           // xa
}

impl FromOverride<PermissionsOverride> for Permissions {
    fn from(value: PermissionsOverride) -> Self {
        Permissions {
            user_read: FromOverride::from(value.user_read),
            user_write: FromOverride::from(value.user_write),
            user_execute_file: FromOverride::from(value.user_execute_file),
            user_execute_other: FromOverride::from(value.user_execute_other),
            group_read: FromOverride::from(value.group_read),
            group_write: FromOverride::from(value.group_write),
            group_execute: FromOverride::from(value.group_execute),
            other_read: FromOverride::from(value.other_read),
            other_write: FromOverride::from(value.other_write),
            other_execute: FromOverride::from(value.other_execute),
            special_user_file: FromOverride::from(value.special_user_file),
            special_other: FromOverride::from(value.special_other),
            attribute: FromOverride::from(value.attribute),
        }
    }
}

impl FromOverride<Option<PermissionsOverride>> for Option<Permissions> {
    fn from(value: Option<PermissionsOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Eq, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SizeOverride {
    pub major: Option<StyleOverride>,        // df
    pub minor: Option<StyleOverride>,        // ds

    pub number_byte: Option<StyleOverride>,  // sn nb
    pub number_kilo: Option<StyleOverride>,  // sn nk
    pub number_mega: Option<StyleOverride>,  // sn nm
    pub number_giga: Option<StyleOverride>,  // sn ng
    pub number_huge: Option<StyleOverride>,  // sn nt

    pub unit_byte: Option<StyleOverride>,    // sb ub
    pub unit_kilo: Option<StyleOverride>,    // sb uk
    pub unit_mega: Option<StyleOverride>,    // sb um
    pub unit_giga: Option<StyleOverride>,    // sb ug
    pub unit_huge: Option<StyleOverride>,    // sb ut
}

impl FromOverride<SizeOverride> for Size {
    fn from(value: SizeOverride) -> Self {
        Size {
            major: FromOverride::from(value.major),
            minor: FromOverride::from(value.minor),
            number_byte: FromOverride::from(value.number_byte),
            number_kilo: FromOverride::from(value.number_kilo),
            number_mega: FromOverride::from(value.number_mega),
            number_giga: FromOverride::from(value.number_giga),
            number_huge: FromOverride::from(value.number_huge),
            unit_byte: FromOverride::from(value.unit_byte),
            unit_kilo: FromOverride::from(value.unit_kilo),
            unit_mega: FromOverride::from(value.unit_mega),
            unit_giga: FromOverride::from(value.unit_giga),
            unit_huge: FromOverride::from(value.unit_huge),
        }
    }
}

impl FromOverride<Option<SizeOverride>> for Option<Size> {
    fn from(value: Option<SizeOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug,Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersOverride {
    pub user_you: Option<StyleOverride>,           // uu
    pub user_root: Option<StyleOverride>,          // uR
    pub user_other: Option<StyleOverride>,         // un
    pub group_yours: Option<StyleOverride>,        // gu
    pub group_other: Option<StyleOverride>,        // gn
    pub group_root: Option<StyleOverride>,         // gR
}

impl FromOverride<UsersOverride> for Users {
    fn from(value: UsersOverride) -> Self {
        Users {
            user_you: FromOverride::from(value.user_you),
            user_root: FromOverride::from(value.user_root),
            user_other: FromOverride::from(value.user_other),
            group_yours: FromOverride::from(value.group_yours),
            group_other: FromOverride::from(value.group_other),
            group_root: FromOverride::from(value.group_root),
        }
    }
}

impl FromOverride<Option<UsersOverride>> for Option<Users> {
    fn from(value: Option<UsersOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct LinksOverride {
    pub normal: Option<StyleOverride>,           // lc
    pub multi_link_file: Option<StyleOverride>,  // lm
}

impl FromOverride<LinksOverride> for Links {
    fn from(value: LinksOverride) -> Self {
        Links {
            normal: FromOverride::from(value.normal),
            multi_link_file: FromOverride::from(value.multi_link_file),
        }
    }
}

impl FromOverride<Option<LinksOverride>> for Option<Links> {
    fn from(value: Option<LinksOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug,Eq, PartialEq, Serialize, Deserialize)]
pub struct GitOverride {
    pub new: Option<StyleOverride>,         // ga
    pub modified: Option<StyleOverride>,    // gm
    pub deleted: Option<StyleOverride>,     // gd
    pub renamed: Option<StyleOverride>,     // gv
    pub typechange: Option<StyleOverride>,  // gt
    pub ignored: Option<StyleOverride>,     // gi
    pub conflicted: Option<StyleOverride>,  // gc
}

impl FromOverride<GitOverride> for Git {
    fn from(value: GitOverride) -> Self {
        Git {
            new: FromOverride::from(value.new),
            modified: FromOverride::from(value.modified),
            deleted: FromOverride::from(value.deleted),
            renamed: FromOverride::from(value.renamed),
            typechange: FromOverride::from(value.typechange),
            ignored: FromOverride::from(value.ignored),
            conflicted: FromOverride::from(value.conflicted),
        }
    }
}

impl FromOverride<Option<GitOverride>> for Option<Git> {
    fn from(value: Option<GitOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitRepoOverride {
    pub branch_main: Option<StyleOverride>,  //Gm
    pub branch_other: Option<StyleOverride>, //Go
    pub git_clean: Option<StyleOverride>,    //Gc
    pub git_dirty: Option<StyleOverride>,    //Gd
}

impl FromOverride<GitRepoOverride> for GitRepo {
    fn from(value: GitRepoOverride) -> Self {
        GitRepo {
            branch_main: FromOverride::from(value.branch_main),
            branch_other: FromOverride::from(value.branch_other),
            git_clean: FromOverride::from(value.git_clean),
            git_dirty: FromOverride::from(value.git_dirty),
        }
    }
}

impl FromOverride<Option<GitRepoOverride>> for Option<GitRepo> {
    fn from(value: Option<GitRepoOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct SELinuxContextOverride {
    pub colon: Option<StyleOverride>,
    pub user: Option<StyleOverride>,  // Su
    pub role: Option<StyleOverride>,  // Sr
    pub typ: Option<StyleOverride>,   // St
    pub range: Option<StyleOverride>, // Sl
}

impl FromOverride<SELinuxContextOverride> for SELinuxContext {
    fn from(value: SELinuxContextOverride) -> Self {
        SELinuxContext {
            colon: FromOverride::from(value.colon),
            user: FromOverride::from(value.user),
            role: FromOverride::from(value.role),
            typ: FromOverride::from(value.typ),
            range: FromOverride::from(value.range),
        }
    }
}

impl FromOverride<Option<SELinuxContextOverride>> for Option<SELinuxContext> {
    fn from(value: Option<SELinuxContextOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Eq, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityContextOverride {
    pub none:    Option<StyleOverride>, // Sn
    pub selinux: Option<SELinuxContextOverride>,
}

impl FromOverride<SecurityContextOverride> for SecurityContext {
    fn from(value: SecurityContextOverride) -> Self {
        SecurityContext {
            none: FromOverride::from(value.none),
            selinux: FromOverride::from(value.selinux),
        }
    }
}

impl FromOverride<Option<SecurityContextOverride>> for Option<SecurityContext> {
    fn from(value: Option<SecurityContextOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct FileTypeOverride {
    pub image: Option<StyleOverride>,       // im - image file
    pub video: Option<StyleOverride>,       // vi - video file
    pub music: Option<StyleOverride>,       // mu - lossy music
    pub lossless: Option<StyleOverride>,    // lo - lossless music
    pub crypto: Option<StyleOverride>,      // cr - related to cryptography
    pub document: Option<StyleOverride>,    // do - document file
    pub compressed: Option<StyleOverride>,  // co - compressed file
    pub temp: Option<StyleOverride>,        // tm - temporary file
    pub compiled: Option<StyleOverride>,    // cm - compilation artifact
    pub build: Option<StyleOverride>,       // bu - file that is used to build a project
    pub source: Option<StyleOverride>,      // sc - source code
}

impl FromOverride<FileTypeOverride> for FileType {
    fn from(value: FileTypeOverride) -> Self {
        FileType {
            image: FromOverride::from(value.image),
            video: FromOverride::from(value.video),
            music: FromOverride::from(value.music),
            lossless: FromOverride::from(value.lossless),
            crypto: FromOverride::from(value.crypto),
            document: FromOverride::from(value.document),
            compressed: FromOverride::from(value.compressed),
            temp: FromOverride::from(value.temp),
            compiled: FromOverride::from(value.compiled),
            build: FromOverride::from(value.build),
            source: FromOverride::from(value.source),
        }
    }
}

impl FromOverride<Option<FileTypeOverride>> for Option<FileType> {
    fn from(value: Option<FileTypeOverride>) -> Self {
        value.map(FromOverride::from)
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UiStylesOverride {
    pub colourful: Option<bool>,

    pub filekinds:        Option<FileKindsOverride>,
    pub perms:            Option<PermissionsOverride>,
    pub size:             Option<SizeOverride>,
    pub users:            Option<UsersOverride>,
    pub links:            Option<LinksOverride>,
    pub git:              Option<GitOverride>,
    pub git_repo:         Option<GitRepoOverride>,
    pub security_context: Option<SecurityContextOverride>,
    pub file_type:        Option<FileTypeOverride>,

    pub punctuation:  Option<StyleOverride>,          // xx
    pub date:         Option<StyleOverride>,          // da
    pub inode:        Option<StyleOverride>,          // in
    pub blocks:       Option<StyleOverride>,          // bl
    pub header:       Option<StyleOverride>,          // hd
    pub octal:        Option<StyleOverride>,          // oc
    pub flags:        Option<StyleOverride>,          // ff

    pub symlink_path:         Option<StyleOverride>,  // lp
    pub control_char:         Option<StyleOverride>,  // cc
    pub broken_symlink:       Option<StyleOverride>,  // or
    pub broken_path_overlay:  Option<StyleOverride>,  // bO

    pub icons: Option<IconStylesOverride>,
}

impl From<UiStylesOverride> for UiStyles {
    fn from(value: UiStylesOverride) -> Self {
        UiStyles {
            colourful: value.colourful,

            filekinds: FromOverride::from(value.filekinds),
            perms: FromOverride::from(value.perms),
            size: FromOverride::from(value.size),
            users: FromOverride::from(value.users),
            links: FromOverride::from(value.links),
            git: FromOverride::from(value.git),
            git_repo: FromOverride::from(value.git_repo),
            security_context: FromOverride::from(value.security_context),
            file_type: FromOverride::from(value.file_type),

            punctuation: FromOverride::from(value.punctuation),
            date: FromOverride::from(value.date),
            inode: FromOverride::from(value.inode),
            blocks: FromOverride::from(value.blocks),
            header: FromOverride::from(value.header),
            octal: FromOverride::from(value.octal),
            flags: FromOverride::from(value.flags),

            symlink_path: FromOverride::from(value.symlink_path),
            control_char: FromOverride::from(value.control_char),
            broken_symlink: FromOverride::from(value.broken_symlink),
            broken_path_overlay: FromOverride::from(value.broken_path_overlay),

            icons: FromOverride::from(value.icons),
        }
    }
}

impl FromOverride<Option<UiStylesOverride>> for Option<UiStyles> {
    fn from(value: Option<UiStylesOverride>) -> Self {
        value.map(From::from)
    }
}

impl ThemeConfig {
    pub fn from_path(path: &str) -> Self {
        let path = PathBuf::from(path);
        ThemeConfig {
            location: ConfigLoc::Env(path),
        }
    }
    pub fn to_theme(&self) -> Option<UiStyles> {
        let ui_styles_override: Option<UiStylesOverride> = match &self.location {
            ConfigLoc::Default => {
                let path = dirs::config_dir()?.join("eza").join("theme.yml");
                let file = std::fs::File::open(path).ok()?;
                serde_yaml::from_reader(&file).ok()
            }
            ConfigLoc::Env(path) => {
                let file = std::fs::File::open(path).ok()?;
                serde_yaml::from_reader(&file).ok()
            }
        };
        FromOverride::from(ui_styles_override)
    }
}
