use num_bigint::ParseBigIntError;
use semver::SemVerError;
use std::io::Error as IOError;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        IO(IOError);
        ParseAmount(ParseBigIntError);
        ParseSemver(SemVerError);
    }

    errors {
        NotThreadSafe {
            description("Failed ensuring thread safety")
        }
        
        InvalidSize {
            description("Invalid size")
        }
        
        InvalidLength {
            description("Invalid length")
        }

        DuplicatedElements {
            description("Duplicated elements")
        }

        IndexOutOfRange {
            description("Index out of range")
        }

        InvalidSum {
            description("Invalid sum")
        }

        InvalidChecksum {
            description("Invalid checksum")
        }

        InvalidBits {
            description("Invalid bits")
        }

        InvalidSCost {
            description("Invalid s cost")
        }

        InvalidTCost {
            description("Invalid t cost")
        }

        InvalidDelta {
            description("Invalid delta")
        }

        InvalidSegmentsRoot {
            description("Invalid segments root")
        }

        InvalidPOW {
            description("Invalid POW")
        }

        InvalidAddress {
            description("Invalid address")
        }

        InvalidSignature {
            description("Invalid signature")
        }

        InvalidAmount {
            description("Invalid amount")
        }

        InvalidTime {
            description("Invalid time")
        }

        InvalidVersion {
            description("Invalid version")
        }

        NotFound {
            description("Not found")
        }

        AlreadyFound {
            description("Already found")
        }

        InvalidId {
            description("Invalid id")
        }

        InvalidPrevBlock {
            description("Invalid prev block")
        }

        InvalidCoinbase {
            description("Invalid coinbase")
        }
    }
}
