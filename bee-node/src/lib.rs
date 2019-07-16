#![deny(bad_style, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, )]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use bee_transaction::SharedTransaction;
use std::sync::Arc;
use std::sync::Mutex;
use bitvec::BitVec;
use std::collections::HashMap;

const MAX_NEIGHBORS : usize = 10;

/// Allows flagging.
pub struct Flagger {
    bit_vec : BitVec
}

impl Flagger {

    pub fn new(flags : usize) -> Self {
        let mut bit_vec = BitVec::new();
        for _ in 0..flags {
            bit_vec.push(false);
        }
        Flagger { bit_vec }
    }

    pub fn set_flag(&mut self, index : usize, value : bool) {
        self.bit_vec.set(index, value);
    }
    pub fn is_flagged(&self, index : usize) -> bool {
        self.bit_vec.get(index)
    }
}

/// A specific use case for Flagger where each flag marks whether a specific neighbor is aware of the
/// transaction. One instance keeps track of one transaction.
type TransactionTracker = Flagger;

/// Access point for all transaction trackers. Being used by both Sender and Receiver.
pub struct GossipTracker {
    // contains data about which neighbor already knows a certain tx
    transaction_tracker_by_hash: HashMap<String, TransactionTracker>
}

impl GossipTracker {

    pub fn new() -> Self {
        GossipTracker {
            transaction_tracker_by_hash : HashMap::new()
        }
    }

    fn get_transaction_tracker(&mut self, hash : &String) -> &mut TransactionTracker {
        self.transaction_tracker_by_hash.entry(hash.clone()).or_insert_with(|| TransactionTracker::new(MAX_NEIGHBORS))
    }
}

/// Responsible for incoming traffic (validating received transaction bytes, listening for requests).
pub struct Receiver {
    sharable_gossip_tracker: Arc<Mutex<GossipTracker>>
}

impl Receiver {

    pub fn new(sharable_gossip_tracker : Arc<Mutex<GossipTracker>>) -> Self {
        Receiver{ sharable_gossip_tracker }
    }

    // TODO add receive() for raw bytes

    /// logic handling a received transaction after validation
    pub fn accept_incoming_transaction(&mut self, transaction : SharedTransaction, sender_index : usize) {
        use std::ops::DerefMut;
        let mut guard = self.sharable_gossip_tracker.lock().unwrap();
        let transaction_tracker : &mut TransactionTracker = &mut guard.deref_mut().get_transaction_tracker(&transaction.address);
        transaction_tracker.set_flag(sender_index, true);
    }
}

/// Responsible for outgoing traffic (broadcasting new transactions, forwarding gossip, answering requests).
pub struct Sender {
    sharable_gossip_tracker : Arc<Mutex<GossipTracker>>
}

impl Sender {

    pub fn new(sharable_gossip_tracker : Arc<Mutex<GossipTracker>>) -> Self {
        Sender{sharable_gossip_tracker}
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
pub fn create_sender_and_receiver() -> (Sender, Receiver) {
    let sharable_gossip_tracker= Arc::new(Mutex::new(GossipTracker::new()));
    let sender = Sender::new(sharable_gossip_tracker.clone());
    let receiver = Receiver::new(sharable_gossip_tracker.clone());
    (sender, receiver)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_transaction_tracking() {
        use bee_transaction::TransactionBuilder;

        let hash = "ABC";
        let stx : SharedTransaction = TransactionBuilder::new().address(&hash).build().into();

        let (mut sender, mut receiver) = create_sender_and_receiver();
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