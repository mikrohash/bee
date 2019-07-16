//#![deny(bad_style, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, )]
//#![cfg_attr(not(debug_assertions), deny(warnings))]

//! This crate provides the sender/receiver logic. It abstracts from the transmission protocol and
//! bridges together the pure data transmission with the internal state of bee. For more information,
//! read the README.md

use bee_transaction::SharedTransaction;
use std::sync::Arc;
use std::sync::Mutex;
use bitvec::BitVec;
use std::collections::HashMap;

pub mod eee;

const MAX_NEIGHBORS : usize = 10;

/// Allows flagging.
pub struct Flagger {
    bit_vec : BitVec
}

impl Flagger {

    /// Creates a new Flagger instance with a custom amount of flags, pre-initialized with `false`.
    pub fn new(flags : usize) -> Self {
        let mut bit_vec = BitVec::new();
        for _ in 0..flags {
            bit_vec.push(false);
        }
        Flagger { bit_vec }
    }

    /// Sets a certain flag to either `true` or `false`.
    /// # Panics
    /// Panics if the index is larger than the amount of reserved flags.
    pub fn set_flag(&mut self, index : usize, value : bool) {
        self.bit_vec.set(index, value);
    }

    /// Returns whether a certain flag is set.
    /// # Panics
    /// Panics if the index is larger than the amount of reserved flags.
    pub fn is_flagged(&self, index : usize) -> bool {
        self.bit_vec.get(index)
    }
}

/// A specific use case for Flagger where each flag marks whether a specific neighbor is aware of the
/// transaction. One instance keeps track of one transaction.
type TransactionTracker = Flagger;

/// Access point for all transaction trackers. Each bee node must use its own GossipTracker but the
/// Sender and Receiver of the same node must share the same GossipTracker instance.
pub struct GossipTracker {
    // contains data about which neighbor already knows a certain tx
    transaction_tracker_by_hash: HashMap<String, TransactionTracker>
}

impl GossipTracker {

    /// Creates a new and empty gossip tracker.
    pub fn new() -> Self {
        GossipTracker {
            transaction_tracker_by_hash : HashMap::new()
        }
    }

    /// Returns the tracker for a specific transaction hash. Creates a new one if not existent.
    fn get_transaction_tracker(&mut self, hash : &String) -> &mut TransactionTracker {
        self.transaction_tracker_by_hash.entry(hash.clone()).or_insert_with(|| TransactionTracker::new(MAX_NEIGHBORS))
    }
}

/// Responsible for incoming traffic (validating received transaction bytes, listening for requests).
pub struct Receiver {
    env_gossip_receive_tx : eee::SharedEnvironment<SharedTransaction>,
    sharable_gossip_tracker: Arc<Mutex<GossipTracker>>
}

impl Receiver {

    fn new(session : &eee::EEESession, sharable_gossip_tracker : Arc<Mutex<GossipTracker>>) -> Self {
        Receiver{
            env_gossip_receive_tx : eee::SUPERVISOR_TX.get_environment(session, "GOSSIP_RECEIVE_TX"),
            sharable_gossip_tracker
        }
    }

    // TODO add receive() for raw bytes

    /// logic handling a received transaction after validation
    pub fn accept_incoming_transaction(&mut self, transaction : SharedTransaction, sender_index : usize) {
        use std::ops::DerefMut;
        let mut guard = self.sharable_gossip_tracker.lock().unwrap();
        let transaction_tracker : &mut TransactionTracker = &mut guard.deref_mut().get_transaction_tracker(&transaction.address);
        transaction_tracker.set_flag(sender_index, true);
        self.env_gossip_receive_tx.send_effect(transaction);
    }
}

/// Responsible for outgoing traffic (broadcasting new transactions, forwarding gossip, answering requests).
pub struct Sender {
    env_transport_forward_tx_bytes : eee::SharedEnvironment<bee_transaction::TransactionBytes>,
    sharable_gossip_tracker : Arc<Mutex<GossipTracker>>
}

impl Sender {

    fn new(session : &eee::EEESession, sharable_gossip_tracker : Arc<Mutex<GossipTracker>>) -> Self {
        Sender{
            env_transport_forward_tx_bytes : eee::SUPERVISOR_TX_BYTES.get_environment(session, "TRANSPORT_FORWARD_TX_BYTES"),
            sharable_gossip_tracker
        }
    }

    fn forward(&self, transaction : SharedTransaction) {
        let bytes = transaction.as_bytes();
        self.env_transport_forward_tx_bytes.send_effect(bytes);
    }

    /// Forwards a transaction to all neighbors who supposedly don't have that transaction yet.
    pub fn forward_transaction(&mut self, transaction : SharedTransaction) {
        use std::ops::DerefMut;
        let mut guard = self.sharable_gossip_tracker.lock().unwrap();
        let transaction_tracker : &mut TransactionTracker = &mut guard.deref_mut().get_transaction_tracker(&transaction.address);

        for i in 0..MAX_NEIGHBORS {
            if !transaction_tracker.is_flagged(i) {
                println!("sending transaction to {}", i);
                transaction_tracker.set_flag(i, true);
            }
        }
    }
}

/// Allows the creation of a Sender and Receiver pair that can be used for the same bee node.
pub fn create_sender_and_receiver(session : &crate::eee::EEESession) -> (Sender, Receiver) {
    let sharable_gossip_tracker= Arc::new(Mutex::new(GossipTracker::new()));
    let sender = Sender::new(session,sharable_gossip_tracker.clone());
    let receiver = Receiver::new(session,sharable_gossip_tracker);
    (sender, receiver)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_transaction_tracking() {
        use bee_transaction::TransactionBuilder;

        let session = &eee::EEESession::new();
        let hash = "ABC";
        let stx : SharedTransaction = TransactionBuilder::new().address(&hash).build().into();

        let (mut sender, mut receiver) = create_sender_and_receiver(session);
        let sharable_gossip_tracker = &sender.sharable_gossip_tracker.clone();

        // transaction not yet received from or forwarded to any neighbor
        assert_transaction_track(&sharable_gossip_tracker, hash, &|_| {false});

        // transaction was received from neighbors 3 and 7
        receiver.accept_incoming_transaction(stx.clone(), 3);
        receiver.accept_incoming_transaction(stx.clone(), 7);
        assert_transaction_track(&sharable_gossip_tracker, hash, &|index| {index == 3 || index == 7});

        // forward transaction to all other neighbors
        sender.forward_transaction(stx);
        assert_transaction_track(&sharable_gossip_tracker, hash, &|_| {true});
    }

    fn assert_transaction_track(sharable_gossip_tracker : &Arc<Mutex<GossipTracker>>, tx_hash : &str, expected : &Fn(usize) -> bool) {
        let mut guarded_gossip_tracker = sharable_gossip_tracker.lock().unwrap();
        let transaction_tracker = guarded_gossip_tracker.get_transaction_tracker(&tx_hash.to_string());

        for i in 0..MAX_NEIGHBORS {
            assert_eq!(expected(i), transaction_tracker.is_flagged(i));
        }
    }
}