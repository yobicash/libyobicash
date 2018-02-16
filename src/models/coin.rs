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
use utils::{Version, NetworkType, Amount};
use models::output::Output;

/// The type used to represent the source of the coin.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CoinSource {
    TransactionFee=0,
    TransactionOutput=1,
    WriteOpFee=2,
    DeleteOpFee=3,
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
            3 => Ok(CoinSource::DeleteOpFee),
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
    /// The output id.
    pub id: Digest,
    /// The protocol version.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The instance used to build the output.
    pub instance: Scalar,
    /// The witness used in the output.
    pub witness: ZKPWitness,
    /// The output amount.
    pub amount: Amount,
    /// The source of the coin.
    pub source: CoinSource,
    /// The id of the source.
    pub source_id: Digest,
}

impl Coin {
    /// Creates a new `Coin`.
    pub fn new(source: CoinSource, source_id: Digest, output: &Output, instance: Scalar) -> Result<Coin> {
        output.validate()?;
        instance.validate()?;

        let witness = ZKPWitness::new(instance)?;
        if witness != output.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }

        let coin = Coin {
            id: output.id,
            version: output.version.clone(),
            network_type: output.network_type,
            instance: instance,
            witness: witness,
            amount: output.amount.clone(),
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
        Ok(self.id)
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
       Ok(id.to_bytes()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(self.id.to_bytes()?)
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
        self.instance.validate()?;
        self.witness.validate()?;
        self.amount.validate()?;

        let witness = ZKPWitness::new(self.instance)?;

        if witness != self.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }
    
        Ok(())
    }
}

impl<'a> Serialize<'a> for Coin {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "instance": self.instance.to_hex()?,
            "witness": self.witness.to_hex()?,
            "amount": self.amount.to_string(),
            "source": self.source.to_hex()?,
            "source_id": self.source_id.to_hex()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_hex: String = json::from_value(id_value)?;
        let id = Digest::from_hex(&id_hex)?;
        
        let version_value = obj["version"].clone();
        let version_str: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_str)?;
        
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;
        
        let instance_value = obj["instance"].clone();
        let instance_hex: String = json::from_value(instance_value)?;
        let instance = Scalar::from_hex(&instance_hex)?;
        
        let witness_value = obj["witness"].clone();
        let witness_hex: String = json::from_value(witness_value)?;
        let witness = ZKPWitness::from_hex(&witness_hex)?;

        let amount_value = obj["amount"].clone();
        let amount_str: String = json::from_value(amount_value)?;
        let amount = Amount::from_string(&amount_str)?;
        
        let source_value = obj["source"].clone();
        let source_hex: String = json::from_value(source_value)?;
        let source = CoinSource::from_hex(&source_hex)?;
        
        let source_id_value = obj["source_id"].clone();
        let source_id_hex: String = json::from_value(source_id_value)?;
        let source_id = Digest::from_hex(&source_id_hex)?;

        let coin = Coin {
            id: id,
            version: version,
            network_type: network_type,
            instance: instance,
            witness: witness,
            amount: amount,
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
