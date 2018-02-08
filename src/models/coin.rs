// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `coin` module provides the coin types and methods.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, Identify, BinarySerialize, HexSerialize, Serialize};
use crypto::{Digest, Scalar, ZKPWitness, ZKPProof};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use crypto::Validate as CryptoValidate;
use models::output::Output;

/// The type used to represent the source of the coin.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CoinSource {
    TransactionFee=0,
    TransactionOutput=1,
    WriteOpFee=2,
    WriteOpOutput=3,
    DeleteOpFee=4,
    DeleteOpOutput=5,
}

impl Default for CoinSource {
    fn default() -> CoinSource {
        CoinSource::TransactionFee
    }
}

impl BinarySerialize for CoinSource {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write_u32::<BigEndian>(*self as u32)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<CoinSource> {
        let len = b.len();
        if len != 4 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let n: u32 = BigEndian::read_u32(b);

        match n {
            0 => Ok(CoinSource::TransactionFee),
            1 => Ok(CoinSource::TransactionOutput),
            2 => Ok(CoinSource::WriteOpFee),
            3 => Ok(CoinSource::WriteOpOutput),
            4 => Ok(CoinSource::DeleteOpFee),
            5 => Ok(CoinSource::DeleteOpOutput),
            _ => Err(ErrorKind::UnknownMode.into()),
        }
    }
}

impl HexSerialize for CoinSource {
    fn from_hex(s: &str) -> Result<CoinSource> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        CoinSource::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}

/// A `Coin` is an `Output` enriched with the instance needed to redeem it.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Coin {
    /// The coin output.
    pub output: Output,
    /// The instance used to redeem the coin.
    pub instance: Scalar,
    /// The source of the coin.
    pub source: CoinSource,
    /// The id of the source.
    pub source_id: Digest,
}

impl Coin {
    /// Creates a new `Coin`.
    pub fn new(output: &Output, instance: Scalar, source: CoinSource, source_id: Digest) -> Result<Coin> {
        output.validate()?;
        instance.validate()?;

        let witness = ZKPWitness::new(instance)?;
        if witness != output.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }

        let coin = Coin {
            output: output.clone(),
            instance: instance,
            source: source,
            source_id: source_id,
        };

        Ok(coin)
    }

    /// Verify the `Coin` against a `ZKPProof`.
    pub fn verify(&self, proof: ZKPProof) -> Result<bool> {
        self.validate()?;
        proof.validate()?;

        let witness = ZKPWitness::new(self.instance)?;

        Ok(proof.verify(witness)?)
    }

    /// Creates a proof from a message.
    pub fn proof(&self, message: &[u8]) -> Result<ZKPProof> {
        self.validate()?;

        Ok(ZKPProof::new(self.instance, message)?)
    }
}

impl<'a> Identify<'a> for Coin {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        Ok(self.output.id)
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
       Ok(id.to_bytes()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(self.output.id.to_bytes()?)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(Digest::from_hex(s)?)
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id.to_hex()?)
    }

    fn string_id(&self) -> Result<String> {
        let id = self.id()?;

        Self::id_to_string(id)
    }
}

impl Validate for Coin {
    fn validate(&self) -> Result<()> {
        self.output.validate()?;
        self.instance.validate()?;

        let witness = ZKPWitness::new(self.instance)?;

        if witness != self.output.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }
    
        Ok(())
    }
}

impl<'a> Serialize<'a> for Coin {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "output": self.output.to_json()?,
            "instance": self.instance.to_hex()?,
            "source": self.source.to_hex()?,
            "source_id": self.source_id.to_hex()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let output_value = obj["output"].clone();
        let output_str: String = json::from_value(output_value)?;
        let output = Output::from_json(&output_str)?;
        
        let instance_value = obj["instance"].clone();
        let instance_hex: String = json::from_value(instance_value)?;
        let instance = Scalar::from_hex(&instance_hex)?;
        
        let source_value = obj["source"].clone();
        let source_hex: String = json::from_value(source_value)?;
        let source = CoinSource::from_hex(&source_hex)?;
        
        let source_id_value = obj["source_id"].clone();
        let source_id_hex: String = json::from_value(source_id_value)?;
        let source_id = Digest::from_hex(&source_id_hex)?;

        let coin = Coin {
            output: output,
            instance: instance,
            source: source,
            source_id: source_id,
        };

        Ok(coin)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let coin = messagepack::from_slice(b)?;

        Ok(coin)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
