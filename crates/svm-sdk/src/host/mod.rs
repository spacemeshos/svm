#[cfg(not(test))]
mod ext;

#[cfg(not(test))]
pub use ext::Host;

#[cfg(test)]
mod mock;

#[cfg(test)]
pub use mock::Host;
