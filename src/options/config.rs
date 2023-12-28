use crate::options::theme::ui_styles::UiStyles;
use nu_ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    display: Option<DisplayOptions>,
    theme: Option<Theme>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayOptions {
    icons: Option<DisplayMode>,
    color: Option<DisplayMode>,
    hyperlinks: Option<DisplayMode>,
    quotes: Option<DisplayMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DisplayMode {
    Always,
    Auto,
    Never,
}

// This will be the base layout for the options in a "Theme" file, so we can separate the
// verbose theme file from the config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeMapping(pub HashMap<ThemeOption, Vec<Color>>);

// This will represent the theme.yaml file itself
#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeMapping,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ThemeOption {
    Directories,
    Executables,
    RegularFiles,
    NamedPipes,
    Sockets,
    BlockDevices,
    CharacterDevices,
    Symlinks,
    Orphans,
    OctalPermissions,
    UserRead,
    UserWrite,
    UserExecuteRegular,
    UserExecuteOther,
    GroupRead,
    GroupWrite,
    GroupExecuteRegular,
    GroupExecuteOther,
    OthersRead,
    OthersWrite,
    OthersExecuteRegular,
    OthersExecuteOther,
    SetIdsStickyBitFiles,
    SetIdsStickyBitOther,
    ExtendedAttribute,
    FileSizeNumbers(Vec<Option<FileSize>>),
    FileSizeUnits(Vec<Option<FileSize>>),
    DeviceMajorId,
    DeviceMinorId,
    UserYou,
    UserRoot,
    UserElse,
    GroupYou,
    GroupRoot,
    GroupElse,
    NumberHardLinks,
    NumberHardLinksRegular,
    GitNew,
    GitModified,
    GitDeleted,
    GitRenamed,
    GitMetadataModified,
    GitIgnored,
    GitConflicted,
    GitMainBranch,
    GitOtherBranch,
    GitClean,
    GitDirty,
    Punctuation,
    FileDate,
    FileInode,
    FileBlocks,
    TableHeader,
    SymlinkPath,
    FilenameEscapedCharacter,
    BrokenSymlinkOverlay,
    Special,
    MountPoint,
    ImageFile,
    VideoFile,
    LossyMusicFile,
    LosslessMusicFile,
    CryptographyFile,
    DocumentFile,
    CompressedFile,
    TemporaryFile,
    CompilationArtifact,
    BuildFile,
    SourceCodeFile,
    NoSecurityContext,
    SelinuxUser,
    SelinuxRole,
    SelinuxType,
    SelinuxLevel,
    BsdFileFlags,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum FileSize {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}
impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            colors: ThemeMapping::default(),
        }
    }
}
