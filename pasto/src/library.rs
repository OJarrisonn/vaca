use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{error::BuildErrorStack, library::{Library, LibraryCollection}};

use crate::BuildResult;

/// Receives a [`Rule::library`] and tries to build a Library
/// This output isn't runtime-ready
/// WIP
pub fn parse_library(library: Pair<Rule>) -> BuildResult<Library> {
    let mut library = library.into_inner();
    let imports = library.next().unwrap();
    let exports = library.next().unwrap();
    let locals = library;

    todo!();
}

/// Receives a [`Rule::imports`] and tries to build a LibraryCollection
/// This output isn't runtime-ready
/// WIP
pub fn parse_library_collection(imports: Pair<Rule>) -> BuildResult<LibraryCollection> {
    todo!();
}