// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind, Read};
use std::path::Path;

use aer_data::prelude::*;
use log::warn;

pub mod errors;
#[cfg(feature = "toml_data")]
pub mod toml;

/// Parsers implementing this trait are able to read and transform a specific
/// structure to the [PackageData] type.
pub trait DataReader {
    /// Function to decide if the implemented structure can handle a certain
    /// file (usually by file extension).
    fn can_handle_file(&self, path: &Path) -> bool;

    /// Read and Deserialize the specified file, calling the implemented
    /// structure that handle the Deserialization.
    fn read_file(&self, path: &Path) -> Result<PackageData, errors::ParserError> {
        if !self.can_handle_file(path) {
            let error = IoError::new(
                ErrorKind::InvalidData,
                format!("The file '{}' is not a supported type.", path.display()),
            );
            warn!("{}", error);
            return Err(errors::ParserError::Loading(error));
        }

        if !path.exists() {
            let error = IoError::new(
                ErrorKind::NotFound,
                format!("The file '{}' could not be found!", path.display()),
            );
            warn!("{}", error);
            return Err(errors::ParserError::Loading(error));
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(error) => return Err(errors::ParserError::Loading(error)),
        };
        let mut buffer = BufReader::new(file);

        self.read_data(&mut buffer)
    }

    /// Read the specifed buffer and return either the parsed package data, or
    /// an error if one occurs.
    fn read_data<T: Read>(&self, reader: &mut T) -> Result<PackageData, errors::ParserError>;
}

#[cfg(any(feature = "toml_data"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "toml_data"))))]
macro_rules! call_parsers {
    ($path:ident,$($parser:expr=>$feature:literal),+) => {
        $(
            #[cfg(feature = $feature)]
            {
                let data = $parser.read_file($path);
                if let Ok(data) = data {
                    return Ok(data);
                } else if $parser.can_handle_file($path) {
                    return data;
                }
            }
        )*
    };
}

#[cfg(any(feature = "toml_data"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "toml_data"))))]
pub fn read_file(path: &Path) -> Result<PackageData, errors::ParserError> {
    call_parsers!(path, toml::TomlParser => "toml_data");

    Err(errors::ParserError::NoParsers(path.to_owned()))
}
