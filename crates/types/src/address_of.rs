use std::marker::PhantomData;

use crate::Address;

/// Holds an `Address` accompanied by a marker type
/// denoting the entity type owning the address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddressOf<T>(PhantomData<T>, Address);

impl<T> AddressOf<T> {
    /// Creates a new struct.
    pub fn new(addr: Address) -> Self {
        Self(PhantomData, addr)
    }

    /// Returns a Borrow to the inner `Address`.
    pub fn inner(&self) -> &Address {
        &self.1
    }

    /// Returns the wrapped `Address`.
    pub fn unwrap(self) -> Address {
        self.1
    }
}

impl<T> From<Address> for AddressOf<T> {
    fn from(addr: Address) -> Self {
        AddressOf::new(addr)
    }
}

impl<T> From<&Address> for AddressOf<T> {
    fn from(addr: &Address) -> Self {
        addr.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum User {}

    #[test]
    fn address_of_inner() {
        let addr = Address::of("someone");
        let user = AddressOf::<User>::new(addr.clone());

        assert_eq!(&addr, user.inner());
    }

    #[test]
    fn address_of_unwrap() {
        let addr = Address::of("someone");
        let user = AddressOf::<User>::new(addr.clone());

        assert_eq!(addr, user.unwrap());
    }

    #[test]
    fn address_of_from() {
        let addr = Address::of("someone");
        let user = AddressOf::<User>::new(addr.clone());

        assert_eq!(user, (&addr).into());
        assert_eq!(user, addr.into());
    }
}
