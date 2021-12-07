use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrownError {
    #[error("path must begin with alphabet, remove ./ if any in the start")]
    PathBeginWithAlphabet,
    #[error("you can only use url safe symbols (-,_,~,/)")]
    NonUrlSafeSymbolFound,
    #[error("could not extract file name (stem)")]
    FileNameError,
    #[error("failed to get file extention")]
    FileExtError,
    #[error("found no valid entries in the directory, it may be empty")]
    DirEmpty,
    #[error("failed to extract path from DirEntry")]
    DirEntryPathError,
    #[error("the directory does not exist")]
    DirNotFound,
    #[error("the path already exists, can not be over-written in safe operation")]
    PathAlreadyExists,
    #[error("the file already exists, can not be over-written in safe operation")]
    FileAlreadyExists,
    #[error("failed to create a file,Rust File::create failed")]
    FailedFileCreation,
    #[error("failed to create a directory,Rust fs::create_dir failed")]
    FailedDirCreation,
}
