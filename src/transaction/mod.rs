use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::hash::{YHash64, YDigest64};
use crypto::elliptic::scalar::YScalar;
use input::YInput;
use output::YOutput;
use utxo::YUTXO;
use std::io::{Write, Cursor, Read};

#[derive(Clone, Eq, PartialEq, Default)]
pub struct YTransaction {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub inputs: Vec<YInput>,
    pub outputs: Vec<YOutput>,
}

impl YTransaction {
    pub fn new(utxos: &Vec<YUTXO>, xs: &Vec<YScalar>, outputs: &Vec<YOutput>) -> YResult<YTransaction> {
        let utxos_len = utxos.len();
        if xs.len() != utxos_len {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut xs_copy = xs.clone();
        xs_copy.dedup();
        if xs_copy.len() != utxos_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        let mut utxos_refs = Vec::new();
        for i in 0..utxos_len {
            let inp = utxos[i];
            let refs = (inp.id, inp.idx);
            utxos_refs.push(refs);
        }
        utxos_refs.sort();
        utxos_refs.dedup();
        if utxos_refs.len() != utxos_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        let outputs_len = outputs.len();
        let mut outputs_refs = Vec::new();
        for i in 0..outputs_len {
            let out = outputs[i].clone();
            let refs = YHash64::hash(&out.sender.to_bytes()[..]);
            outputs_refs.push(refs);
        }
        outputs_refs.sort();
        outputs_refs.dedup();
        if outputs_refs.len() != outputs_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        let now = YTime::now();
        let version = YVersion::default();
        let id = YDigest64::default();
        let mut inputs = Vec::new();
        let mut uxs = Vec::new();
        for i in 0..utxos_len {
            let x = xs[i];
            let u = YScalar::random();
            uxs.push(u);
            let c = YScalar::zero();
            let utxo = utxos[i];
            inputs.push(utxo.to_input(x, u, c)?);
        }
        let mut tx = YTransaction {
            id: id,
            version: version,
            time: now,
            inputs: inputs.clone(),
            outputs: outputs.clone(),
        };
        let inputs_len = inputs.len();
        for i in 0..inputs_len {
            let x = xs[i];
            let u = uxs[i];
            let c = tx.calc_challenge(i as u32)?;
            let r = &u + &(&x*&c);
            tx.inputs[i].c = c;
            tx.inputs[i].r = r;
        }

        tx.id = tx.calc_id()?;

        Ok(tx)
    }

    pub fn calc_challenge(&self, idx: u32) -> YResult<YScalar> {
        // a) the idx input is substituted with a default YInput
        // b) all the non-idx inputs' challenges are substituted with the default YScalar
        
        let mut buf = Vec::new();

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        let inputs = self.inputs.clone();
        let inputs_len = inputs.len();

        buf.write_u32::<BigEndian>(inputs_len as u32)?;

        for i in 0..inputs_len {
            let mut input = inputs[i];
            if i == idx as usize {
                input = YInput::default();
            } else {
                input.c = YScalar::default();
            }
            let input_buf = input.to_bytes()?;
            buf.write_u32::<BigEndian>(input_buf.len() as u32)?;
            buf.write(input_buf.as_slice())?;
        }

        let outputs = self.outputs.clone();
        let outputs_len = outputs.len();

        buf.write_u32::<BigEndian>(outputs_len as u32)?;

        for i in 0..outputs_len {
            let output_buf = outputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
            buf.write(output_buf.as_slice())?;
        }
        
        let c = YScalar::hash_from_bytes(buf.as_slice());
        Ok(c)
    }

    pub fn calc_id(&self) -> YResult<YDigest64> {
        let mut buf = Vec::new();

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        let inputs = self.inputs.clone();
        let inputs_len = inputs.len();

        buf.write_u32::<BigEndian>(inputs_len as u32)?;

        for i in 0..inputs_len {
            let input_buf = inputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(input_buf.len() as u32)?;
            buf.write(input_buf.as_slice())?;
        }

        let outputs = self.outputs.clone();
        let outputs_len = outputs.len();

        buf.write_u32::<BigEndian>(outputs_len as u32)?;

        for i in 0..outputs_len {
            let output_buf = outputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
            buf.write(output_buf.as_slice())?;
        }
        Ok(YHash64::hash(buf.as_slice()))
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        let inputs = self.inputs.clone();
        let inputs_len = inputs.len();
        buf.write_u32::<BigEndian>(inputs_len as u32)?;
        for i in 0..inputs_len {
            let input_buf = inputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(input_buf.len() as u32)?;
            buf.write(input_buf.as_slice())?;
        }

        let outputs = self.outputs.clone();
        let outputs_len = outputs.len();
        buf.write_u32::<BigEndian>(outputs_len as u32)?;
        for i in 0..outputs_len {
            let output_buf = outputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
            buf.write(output_buf.as_slice())?;
        }
        
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YTransaction> {
        if b.len() < 104 {
            return Err(YErrorKind::Unknown.into());
        }

        let mut tx = YTransaction::default();

        let mut reader = Cursor::new(b);

        let mut id_buf = [0u8; 64];
        reader.read_exact(&mut id_buf[..])?;
        tx.id = YDigest64::from_bytes(&id_buf[..])?;

        let mut ver_buf = [0u8; 24];
        reader.read_exact(&mut ver_buf[..])?;
        tx.version = YVersion::from_bytes(&ver_buf[..])?;

        let mut time_buf = [0u8; 8];
        reader.read_exact(&mut time_buf[..])?;
        tx.time = YTime::from_bytes(&time_buf[..])?;

        let inputs_len = reader.read_u32::<BigEndian>()? as usize;

        for _ in 0..inputs_len {
            let input_len = reader.read_u32::<BigEndian>()? as usize;
            let mut input_buf = Vec::new();
            for _ in 0..input_len {
                input_buf.push(0);
            }
            reader.read_exact(&mut input_buf.as_mut_slice())?;
            let input = YInput::from_bytes(input_buf.as_slice())?;
            tx.inputs.push(input);
        }

        let outputs_len = reader.read_u32::<BigEndian>()? as usize;

        for _ in 0..outputs_len {
            let output_len = reader.read_u32::<BigEndian>()? as usize;
            let mut output_buf = Vec::new();
            for _ in 0..output_len {
                output_buf.push(0);
            }
            reader.read_exact(&mut output_buf.as_mut_slice())?;
            let output = YOutput::from_bytes(output_buf.as_slice())?;
            tx.outputs.push(output);
        }

        Ok(tx)
    }

    pub fn from_hex(s: &str) -> YResult<YTransaction> {
        let buf = s.from_hex()?;
        YTransaction::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn verify_input(&self, idx: u32, output: &YOutput) -> YResult<bool> {
        if self.inputs.len() < 1 + idx as usize {
            return Err(YErrorKind::Unknown.into());
        }
        Ok(self.inputs[idx as usize].verify(output))
    }

    pub fn verify(&self, outputs: &Vec<YOutput>) -> YResult<bool> {
        let len = self.inputs.len();
        if outputs.len() != len {
            return Err(YErrorKind::InvalidLength.into());
        }
        for idx in 0..len {
            let verified = self.verify_input(idx as u32, &outputs[idx as usize])?;
            if !verified {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn drop_output(mut self, idx: u32) -> YResult<YTransaction> {
        let i = idx as usize;
        if self.outputs.len() - 1 < i {
            return Err(
                YErrorKind::IndexOutOfBound(i as usize, self.outputs.len()).into(),
            );
        }
        self.outputs[i] = self.outputs[i].clone().drop();
        Ok(self)
    }

    pub fn drop_all(mut self) -> YTransaction {
        for i in 0..self.outputs.len() {
            self.outputs[i] = self.outputs[i].clone().drop();
        }
        self
    }
}
