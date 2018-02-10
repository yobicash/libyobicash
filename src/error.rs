// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `error` module provides the errors used throughout the library.

use failure::{Fail, Context, Backtrace};
use failure::Error as FailureError;
use yobicrypto::Error as CryptoError;
use rmp_serde::encode::Error as ToMessagePackError;
use rmp_serde::decode::Error as FromMessagePackError;
use hex::FromHexError;
use serde_json::Error as JsonError;
use chrono::ParseError as FromTimeError;
use regex::Error as RegexError;
use rug::rational::ParseRationalError;

use std::fmt::{self, Display};
use std::string::FromUtf8Error;
use std::num::ParseIntError;
use std::io::Error as IOError;

/// The error type used in `libyobicash`.
#[derive(Debug)]
pub struct Error {
    /// Inner `Context` with the `Fail` implementor.
    inner: Context<ErrorKind>, 
}

/// The different types of errors used in `libyobicash`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display="Already found")]
    AlreadyFound,
    #[fail(display="Not found")]
    NotFound,
    #[fail(display="Out of bound")]
    OutOfBound,
    #[fail(display="Invalid format")]
    InvalidFormat,
    #[fail(display="Not supported")]
    NotSupported,
    #[fail(display="Invalid length")]
    InvalidLength,
    #[fail(display="Found duplicates")]
    DuplicatesFound,
    #[fail(display="Invalid digest")]
    InvalidDigest,
    #[fail(display="Invalid id")]
    InvalidID,
    #[fail(display="Invalid time")]
    InvalidTime,
    #[fail(display="Invalid timestamp")]
    InvalidTimestamp,
    #[fail(display="Invalid version")]
    InvalidVersion,
    #[fail(display="Invalid secret key")]
    InvalidSecretKey,
    #[fail(display="Invalid public key")]
    InvalidPublicKey,
    #[fail(display="Invalid witness")]
    InvalidWitness,
    #[fail(display="Invalid proof")]
    InvalidProof,
    #[fail(display="Unknown mode")]
    UnknownMode,
    #[fail(display="Invalid mode")]
    InvalidMode,
    #[fail(display="Unknown network")]
    UnknownNetwork,
    #[fail(display="Invalid network")]
    InvalidNetwork,
    #[fail(display="Invalid genesis")]
    InvalidGenesis,
    #[fail(display="Crypto failure")]
    CryptoFailure,
    #[fail(display="Regex failure")]
    RegexFailure,
    #[fail(display="Json failure")]
    JsonFailure,
    #[fail(display="Serialization failure")]
    SerializationFailure,
    #[fail(display="Deserialization failure")]
    DeserializationFailure,
    #[fail(display="I/O failure")]
    IOFailure,
    #[fail(display="From Failure")]
    FromFailure,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

impl From<FailureError> for Error {
    fn from(e: FailureError) -> Error {
        Error { inner: e.context(ErrorKind::FromFailure) }
    }
}

impl From<CryptoError> for Error {
    fn from(e: CryptoError) -> Error {
        Error { inner: e.context(ErrorKind::CryptoFailure) }
    }
}

impl From<IOError> for Error {
    fn from(_: IOError) -> Error {
        Error { inner: Context::new(ErrorKind::IOFailure) }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}

impl From<ParseRationalError> for Error {
    fn from(_: ParseRationalError) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}

impl From<RegexError> for Error {
    fn from(_: RegexError) -> Error {
        Error { inner: Context::new(ErrorKind::RegexFailure) }
    }
}

impl From<ToMessagePackError> for Error {
    fn from(_: ToMessagePackError) -> Error {
        Error { inner: Context::new(ErrorKind::SerializationFailure) }
    }
}

impl From<FromMessagePackError> for Error {
    fn from(_: FromMessagePackError) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}

impl From<JsonError> for Error {
    fn from(_: JsonError) -> Error {
        Error { inner: Context::new(ErrorKind::JsonFailure) }
    }
}

impl From<FromHexError> for Error {
    fn from(_: FromHexError) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}

impl From<FromTimeError> for Error {
    fn from(_: FromTimeError) -> Error {
        Error { inner: Context::new(ErrorKind::DeserializationFailure) }
    }
}
