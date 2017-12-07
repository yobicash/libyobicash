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

    InvalidBalloonSCost {
        description("Invalid Balloon s cost parameter")
    }

    InvalidBalloonTCost {
        description("Invalid Balloon t cost parameter")
    }

    InvalidDifficulty {
        description("Invalid difficulty")
    }

    PoWDigestNotFound {
        description("PoW digest not found")
    }

    InvalidTargetBits {
        description("Invalid target bits")
    }

    PoStNotFound {
        description("PoSt not found")
    }

    IncompletePoW {
        description("Incomplete PoW")
    }

    InvalidPoWSolution {
        description("Invalid PoW solution")
    }

    PoWNotFound {
        description("PoW not found")
    }
  }
}
