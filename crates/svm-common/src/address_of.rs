use std::marker::PhantomData;

use crate::Address;

#[derive(Debug, PartialEq)]
pub struct AddressOf<T>(PhantomData<T>, Address);

impl<T> AddressOf<T> {
    pub fn new(addr: Address) -> Self {
        Self(PhantomData, addr)
    }

    pub fn inner(&self) -> &Address {
        &self.1
    }

    pub fn unwrap(self) -> Address {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum User {}

    #[test]
    fn address_of_enum() {
        let addr = Address::of("someone");
        let user = AddressOf::<User>::new(addr.clone());

        assert_eq!(&addr, user.inner());
        assert_eq!(addr, user.unwrap());
    }
}
