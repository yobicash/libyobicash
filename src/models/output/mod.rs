use errors::*;
use models::amount::Amount;
use models::address::Address;
use models::address::check_address;
use models::content::Content;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Output {
    to: Address,
    amount: Amount,
    content: Content,
}

impl Output {
    pub fn new(amount: &Amount, to: &Address, content: &Content) -> Result<Self> {
        check_address(to)?;
        content.check()?;
        let size = content.get_size();
        if size > 0 && Amount::new(size) != amount.to_owned() {
                return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(Output {
            to: to.to_owned(),
            amount: amount.to_owned(),
            content: content.to_owned(),
        })
    }

    pub fn get_to(&self) -> Address {
        self.to.to_owned()
    }

    pub fn set_to(&mut self, to: &Address) -> Result<Self> {
        check_address(to)?;
        self.to = to.to_owned();
        Ok(self.to_owned())
    }

    pub fn get_amount(&self) -> Amount {
        self.amount.to_owned()
    }

    pub fn set_amount(&mut self, amount: &Amount) -> Result<Self> {
        self.amount = amount.to_owned();
        Ok(self.to_owned())
    }

    pub fn get_content(&self) -> Content {
        self.content.to_owned()
    }

    pub fn set_content(&mut self, content: &Content) -> Result<Self> {
        content.check()?;
        self.content = content.to_owned();
        Ok(self.to_owned())
    }

    pub fn check(&self) -> Result<()> {
        check_address(&self.to)?;
        self.content.check()?;
        let size = self.content.get_size();
        if size > 0 && Amount::new(size) != self.amount.to_owned() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.to.as_slice())?;
        bin.write_all(self.amount.to_vec().as_slice())?;
        bin.write_all(self.content.to_vec()?.as_slice())?;
        Ok(bin)
    }
}
