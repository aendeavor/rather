/// Provide initial default values so the user may not need to
/// specify values.
pub mod defaults;

/// All data structures related to L2 are contained in the
/// `frames` module.
pub mod frames;

/// Provides the abstractions over creating raw sockets and
/// sending frames on L2 over them.
pub mod sockets;

/// Provides abstractions over user input, parsing and output.
pub mod interaction;

/// Stands for the packet family (domain).
const AF_PACKET: i32 = 17;

/// Is the same as `AF_PACKET`.
const PF_PACKET: u16 = 17;

/// Stands for the socket type.
const SOCK_RAW: i32 = 3;

/// Represents the length of MAC addresses.
const ETH_MAC_ADDRESS_LENGTH: usize = 6;

/// Represents the length of MAC addresses.
const ETH_PROTOCOL_ADDRESS_FIELD_LENGTH: usize = 2;

/// Represents the maximum payload of an Ethernet frame.
#[allow(dead_code)]
const ETH_HEADER_LENGTH: usize =
	ETH_MAC_ADDRESS_LENGTH * 2 + ETH_PROTOCOL_ADDRESS_FIELD_LENGTH;
