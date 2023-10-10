// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum ErrorKind {
    Bug,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CsYamlError {
    pub(crate) kind: ErrorKind,
    pub(crate) msg: String,
}

impl CsYamlError {
    pub(crate) fn new(kind: ErrorKind, msg: String) -> Self {
        Self { kind, msg }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for CsYamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl From<std::io::Error> for CsYamlError {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Bug, format!("std::io::Error: {}", e))
    }
}

impl From<std::ffi::NulError> for CsYamlError {
    fn from(e: std::ffi::NulError) -> Self {
        Self::new(ErrorKind::Bug, format!("std::ffi::NulError: {}", e))
    }
}

impl From<serde_yaml::Error> for CsYamlError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::new(ErrorKind::Bug, format!("serde_yaml::Error: {}", e))
    }
}
