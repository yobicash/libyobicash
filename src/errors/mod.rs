use std::io::Error as IOError;

error_chain! {
  types {
    YError, YErrorKind, YResultExt, YResult;
  }

  links {}

  foreign_links {
    IO(IOError);
  }

  errors {
    InvalidLength(exp: usize, act: usize) {
      description("Invalid length")
      display("Expected length {}, found {}", exp, act)
    }

    IndexOutOfBound(idx: usize, length: usize) {
      description("Index out of bound")
      display("Index {} out of bound in array of length {}", idx, length)
    }

    ParseBigInt(s: String) {
      description("Bigint string parsing error")
      display("Unable to parse the bigint string {}", s)
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
