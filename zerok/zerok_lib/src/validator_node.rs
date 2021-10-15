use crate::{ElaboratedBlock, ElaboratedTransaction, ValidatorState};

use phaselock::{
    message::Message, networking::NetworkingImplementation, NodeImplementation, Storage, H_512,
};

use core::fmt::Debug;
use core::marker::PhantomData;

/// A lightweight node that handles validation for consensus, and nothing more.
/// TODO: replace with persisting version of ValidatorNodeImpl, complete with handler for decide callback;
pub trait PLNet:
    NetworkingImplementation<Message<ElaboratedBlock, ElaboratedTransaction, H_512>>
    + Clone
    + Debug
    + 'static
{
}

impl<
        T: NetworkingImplementation<Message<ElaboratedBlock, ElaboratedTransaction, H_512>>
            + Clone
            + Debug
            + 'static,
    > PLNet for T
{
}

pub trait PLStore:
    Storage<ElaboratedBlock, ValidatorState, H_512> + Clone + Send + Sync + 'static
{
}

impl<T: Storage<ElaboratedBlock, ValidatorState, H_512> + Clone + Send + Sync + 'static> PLStore
    for T
{
}

#[derive(Clone)]
pub struct ValidatorNodeImpl<NET: PLNet, STORE: PLStore> {
    net: PhantomData<NET>,
    store: PhantomData<STORE>,
}

impl<NET: PLNet, STORE: PLStore> Debug for ValidatorNodeImpl<NET, STORE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidatorNodeImpl").finish()
    }
}

impl<NET: PLNet, STORE: PLStore> NodeImplementation<H_512> for ValidatorNodeImpl<NET, STORE> {
    type Block = ElaboratedBlock;

    type State = ValidatorState;

    type Storage = STORE;

    type Networking = NET;
}
