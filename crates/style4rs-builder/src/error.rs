use std::io;

pub enum Error {
    FailedToReadFile(io::Error),
    FailedToParseFile(syn::Error),
    FailedToCreateTargetFolder(io::Error),
    FailedToWriteTargetFile(io::Error),
    InFolderPathNotDefined,
    OutFilePathNotDefined,
}
