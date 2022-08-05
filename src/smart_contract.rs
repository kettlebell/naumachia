use crate::{error::Result, Address, UnBuiltTransaction};
use std::hash::Hash;

pub trait SmartContract {
    type Endpoint;
    type Datum;
    type Redeemer: Clone + PartialEq + Eq + Hash;

    fn handle_endpoint(
        endpoint: Self::Endpoint,
        issuer: &Address,
    ) -> Result<UnBuiltTransaction<Self::Datum, Self::Redeemer>>;
}
