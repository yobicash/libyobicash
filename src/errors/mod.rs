use serialize::hex::FromHexError;
use semver::SemVerError;
use num_bigint::ParseBigIntError as BigUintError;
use std::io::Error as IOError;

error_chain! {
  types {
    YError, YErrorKind, YResultExt, YResult;
  }

  links {}

  foreign_links {
    IO(IOError);
    Hex(FromHexError);
    Version(SemVerError);
    BigUint(BigUintError);
  }

  errors {
    InvalidLength {
      description("Invalid length")
    }

    IndexOutOfBound(idx: usize, length: usize) {
      description("Index out of bound")
      display("Index {} out of bound in array of length {}", idx, length)
    }

    ParseBigUint(s: String) {
      description("Biguint parsing error")
      display("Unable to parse the biguint: {}", s)
    }

    BigUintOutOfBound {
        description("BigUint out ouf bound")
    }

    InvalidPoint(reason: String) {
      description("Invalid point")
      display("Invalid point: {}", reason)
    }

    InvalidCyph {
      description("Invalid cyphertext")
    }

    InvalidHeight {
      description("Invalid height")
    }

    AmountOutOfBound {
      description("Amount out of bound")
    }

    DuplicateItem {
      description("Duplicate item")
    }

    InvalidAmount {
        description("Invalid amount")
    }

    InvalidChecksum {
        description("Invalid checksum")
    }

    InvalidChallenge(idx: usize) {
      description("Invalid challenge")
      display("Invalid challenge at index {}", idx)
    }

    InvalidVersion(v: String) {
        description("Invalid version")
        display("Invalid version: {}", v)
    }

    InvalidTime {
        description("Invalid time")
    }

    InvalidActivation {
        description("Invalid activation")
    }

    InvalidBalloonDelta {
        description("Invalid Balloon delta parameter")
    }

    InvalidBalloonTCost {
        description("Invalid Balloon t cost parameter")
    }
  }
}
