#[cfg(not(test))]
mod ext;

#[cfg(not(test))]
pub use ext::Storage;

#[cfg(test)]
mod mock;

#[cfg(test)]
pub use mock::Storage;
