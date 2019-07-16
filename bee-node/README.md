# `bee-node`

This crate is supposed to handle the sender/receiver logic. As a layer inbetween, it bridges together:
 - the external data transmission from and to neighbors (`bee-communication`)
 - and the internal state of the bee (mostly `bee-tangle`)
 
 It abstracts from the technical communication and is therefore not aware of the transmission protocols in use. Instead, it processes
 incoming transaction bytes, handles requests and forwards transaction gossip to neighbors. To do that, it must also keep track of which
 neighbor supposedly knows which transactions to reduce redundant transmission.
 
 # Environments
 
Environment | Type | L/P* | Description
-- | -- | -- | --
`TRANSPORT_RECEIVE_BYTES` | `(Bytes,Neighbor)` | L | Unvalidated bytes received from neighbors. Should decode to transaction or request.
`GOSSIP_FORWARD_TX` | `SharedTransaction` | L | Transactions supposed to be forwarded - e.g. new transactions issued by own bee.
`GOSSIP_ISSUE_REQUEST` | `TransactionHash` | L | Transaction requests originating from own bee to be sent to neighbors.
`TRANSPORT_SEND_BYTES` | `(Bytes,Neighbor)` | P | Outgoing traffic. Bytes are sent to a neighbor.
`GOSSIP_RECEIVE_TX` | `SharedTransaction` | P | Validated transaction received from neighbor.

\* `L/P` ... `L`: listens to environment; `P`: publishes to environment.
