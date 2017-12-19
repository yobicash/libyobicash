use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::keys::*;
use amount::YAmount;
use input::YInput;
use output::YOutput;
use utxo::YUTXO;
use std::io::{Write, Cursor, Read};

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YTransaction {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub activation: Option<YTime>,
    pub inputs: Vec<YInput>,
    pub outputs: Vec<YOutput>,
}

impl YTransaction {
    pub fn new(utxos: &Vec<YUTXO>, xs: &Vec<YScalar>, outputs: &Vec<YOutput>, activation: Option<YTime>) -> YResult<YTransaction> {
        let utxos_len = utxos.len();
        let outputs_len = outputs.len();

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
            let utxo = utxos[i].clone();
            let refs = (utxo.id, utxo.idx);
            utxos_refs.push(refs);
        }
        utxos_refs.sort();
        utxos_refs.dedup();
        if utxos_refs.len() != utxos_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        
        let mut outputs_refs = Vec::new();
        for i in 0..outputs_len {
            let out = outputs[i].clone();
            let refs = YSHA512::hash(&out.sender.to_bytes()[..]);
            outputs_refs.push(refs);
        }
        outputs_refs.sort();
        outputs_refs.dedup();
        if outputs_refs.len() != outputs_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        
        let now = YTime::now();
        if let Some(_activation) = activation.clone() {
            if _activation <= now {
                return Err(YErrorKind::InvalidTime.into());
            }
        }

        let version = YVersion::default();

        let id = YDigest64::default();
       
        let mut inputs = Vec::new();
        let mut uxs = Vec::new();
        let mut max_height = 0;
        for i in 0..utxos_len {
            let x = xs[i];
            let u = YScalar::random();
            uxs.push(u);
            let c = YScalar::zero();
            let utxo = utxos[i].clone();
            if max_height < utxo.height {
                max_height = utxo.height;
            }
            inputs.push(utxo.to_input(x, u, c)?);
        }

        if (outputs.len() == 0) ^ (inputs.len() == 0) {
            return Err(YErrorKind::InvalidLength.into());
        }

        for i in 0..outputs_len {
            if outputs[i].height != max_height + 1 {
                return Err(YErrorKind::InvalidHeight.into());
            }
        }
        
        let mut tx = YTransaction {
            id: id,
            version: version,
            time: now,
            activation: activation,
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

    pub fn new_coins(
        main_sk: &YSecretKey,
        change_sk: &YSecretKey,
        main_pk: &YPublicKey,
        change_pk: &YPublicKey,
        amount: YAmount,
        utxos: &Vec<YUTXO>,
        xs: &Vec<YScalar>,
        activation: Option<YTime>,
        message: Option<Vec<u8>>) -> YResult<YTransaction> {
        
        let mut max_amount = YAmount::zero();
        for i in 0..utxos.len() {
            max_amount += utxos[i].amount.clone();
        }

        if amount > max_amount {
            return Err(YErrorKind::InvalidAmount.into());
        }

        let mut max_height = 0;

        for i in 0..utxos.len() {
            if max_height < utxos[i].height {
                max_height = utxos[i].height;
            }
        }

        max_height += 1;

        let coin_out = YOutput::new(main_sk, main_pk, max_height, amount.clone(), message)?;

        let mut outputs = vec![coin_out];

        let change_amount = max_amount - amount;

        if change_amount != YAmount::zero() {
            let change_out = YOutput::new(change_sk, change_pk, max_height, change_amount, None)?;
            outputs.push(change_out);
        }

        YTransaction::new(utxos, xs, &outputs, activation)
    }

    pub fn new_data(data_sk: &YSecretKey,
                    change_sk: &YSecretKey,
                    data_pk: &YPublicKey,
                    change_pk: &YPublicKey,
                    data_buf: &[u8],
                    utxos: &Vec<YUTXO>,
                    xs: &Vec<YScalar>,
                    activation: Option<YTime>,
                    message: Option<Vec<u8>>) -> YResult<YTransaction> {
        let mut buf = Vec::new();
        buf.extend_from_slice(data_buf);

        let padding = data_buf.len() % 16;
        if padding != 0 {
            for _ in 0..padding {
                buf.push(0);
            }
        }

        let mut amount = YAmount::zero();
        for i in 0..utxos.len() {
            amount += utxos[i].amount.clone();
        }

        let mut max_height = 0;

        for i in 0..utxos.len() {
            if max_height < utxos[i].height {
                max_height = utxos[i].height;
            }
        }

        max_height += 1;

        let data_out = YOutput::with_data(&data_sk, data_pk, max_height, &buf, message)?;
        let data_amount = data_out.amount.clone();
        if data_amount > amount {
            return Err(YErrorKind::InvalidAmount.into());
        }

        let mut outputs = vec![data_out];
        
        let change_amount = amount - data_amount;

        if change_amount != YAmount::zero() {
            let change_out = YOutput::new(&change_sk, change_pk, max_height, change_amount, None)?;
            outputs.push(change_out);
        }

        YTransaction::new(utxos, xs, &outputs, activation)
    }

    pub fn new_genesys() -> YResult<YTransaction> {
        YTransaction::new(&vec![], &vec![], &vec![], None)
    }

    pub fn calc_challenge(&self, idx: u32) -> YResult<YScalar> {
        // a) the idx input is substituted with a default YInput
        // b) all the non-idx inputs' challenges are substituted with the default YScalar
        
        let mut buf = Vec::new();

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_activation) = self.activation.clone() {
            buf.write_u32::<BigEndian>(1)?;
            let activation_buf = _activation.to_bytes();
            buf.write(&activation_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

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

        if let Some(_activation) = self.activation.clone() {
            buf.write_u32::<BigEndian>(1)?;
            let activation_buf = _activation.to_bytes();
            buf.write(&activation_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

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
        Ok(YSHA512::hash(buf.as_slice()))
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;

        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_activation) = self.activation.clone() {
            buf.write_u32::<BigEndian>(1)?;
            let activation_buf = _activation.to_bytes();
            buf.write(&activation_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

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
        if b.len() < 96 {
            return Err(YErrorKind::InvalidLength.into());
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

        let has_activation = reader.read_u32::<BigEndian>()?;
        if has_activation == 1 {
            let mut activation_buf = [0u8; 8];
            reader.read_exact(&mut activation_buf[..])?;
            tx.activation = Some(YTime::from_bytes(&activation_buf[..])?);
        }

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

        tx.check()?;

        Ok(tx)
    }

    pub fn from_hex(s: &str) -> YResult<YTransaction> {
        let buf = s.from_hex()?;
        YTransaction::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn verify_input(&self, idx: u32, prev_output: &YOutput) -> YResult<bool> {
        if self.inputs.len() < 1 + idx as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(self.inputs[idx as usize].verify(prev_output))
    }

    pub fn verify(&self, prev_outputs: &Vec<YOutput>) -> YResult<bool> {
        let len = self.inputs.len();
        if prev_outputs.len() != len {
            return Err(YErrorKind::InvalidLength.into());
        }
        for idx in 0..len {
            let verified = self.verify_input(idx as u32, &prev_outputs[idx as usize])?;
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

    pub fn is_dropped(&self) -> bool {
        let mut dropped = true;
        for i in 0..self.outputs.len() {
            dropped &= self.outputs[i].is_dropped();
        }
        dropped
    }

    pub fn is_active(&self) -> bool {
        if let Some(_activation) = self.activation.clone() {
            _activation <= YTime::now()
        } else {
            false
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.id != self.calc_id()? {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        if self.version.major() > YVersion::default().major() {
            let v = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(v).into());
        }

        let time = self.time.clone();
        let now = YTime::now();
        if time > now {
            return Err(YErrorKind::InvalidTime.into())
        }

        if let Some(_activation) = self.activation.clone() {
            if _activation <= time {
                return Err(YErrorKind::InvalidTime.into())
            }
        }

        let mut max_height = 0;

        for i in 0..self.inputs.len() {
            let input = self.inputs[i].clone();
            if input.c != self.calc_challenge(i as u32)? {
                return Err(YErrorKind::InvalidChallenge(i).into());
            }
            let input_height = input.height;
            if max_height < input_height {
                max_height = input_height;
            }
        }

        if (self.outputs.len() == 0) ^ (self.inputs.len() == 0) {
            return Err(YErrorKind::InvalidLength.into());
        }

        for output in self.outputs.clone() {
            if output.height != max_height + 1 {
                return Err(YErrorKind::InvalidHeight.into()); 
            }
            output.check()?;
        }

        Ok(())
    }
}
