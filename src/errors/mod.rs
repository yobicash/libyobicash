use semver::SemVerError;
use std::io::Error as IOError;

error_chain! {
  types {
    YError, YErrorKind, YResultExt, YResult;
  }

  links {}

  foreign_links {
    IO(IOError);
    Version(SemVerError);
  }

  errors {
    InvalidLength {
      description("Invalid length")
    }

    IndexOutOfBound(idx: usize, length: usize) {
      description("Index out of bound")
      display("Index {} out of bound in array of length {}", idx, length)
    }

    ParseBigInt(s: String) {
      description("Bigint string parsing error")
      display("Unable to parse the bigint string {}", s)
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

    InvalidInputChallenge(idx: usize) {
      description("Invalid input challenge")
      display("Invalid input challenge at index {}", idx)
    }

    Unknown {
      description("unknown error")
    }
  }
}
