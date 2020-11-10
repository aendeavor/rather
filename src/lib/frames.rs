use super::{
	super::defaults,
	ETH_MAC_ADDRESS_LENGTH,
	ETH_PROTOCOL_ADDRESS_FIELD_LENGTH,
};

/// # The Ethernet Frame
///
/// The `EthernetFrame` struct represents an Ethernet
/// frame on L2 with all of its field nicely separated.
///
/// ## Important Notes
///
/// This struct is **not** used in C wrappers, but instead
/// serialized into a vector passed to the C wrapper.
///
/// ## Byte Order
///
/// The network byteorder is used, i.e. Big Endian.
#[derive(Debug)]
pub struct EthernetFrame
{
	pub header: EthernetHeader,
	payload:    Vec<u8>,
}

impl From<defaults::Defaults> for EthernetFrame
{
	fn from(information: defaults::Defaults) -> Self
	{
		Self {
			header:  EthernetHeader {
				destination: information.destination,
				source:      information.source,
				protocol:    information.protocol,
			},
			payload: Self::serialize_string_payload_to_ascii(
				&information.data,
			),
		}
	}
}

impl EthernetFrame
{
	pub fn to_array(&self) -> (Vec<u8>, usize)
	{
		let mut serialized_vector: Vec<u8> = vec![];

		for field in &self.header.destination {
			serialized_vector.push(*field);
		}

		for field in &self.header.source {
			serialized_vector.push(*field);
		}

		for field in &self.header.protocol {
			serialized_vector.push(*field);
		}

		for field in &self.payload {
			serialized_vector.push(*field);
		}

		let serialized_vector_length = serialized_vector.len();
		(serialized_vector, serialized_vector_length)
	}

	fn serialize_string_payload_to_ascii(payload: &str) -> Vec<u8>
	{
		let mut ascii_payload: Vec<u8> = vec![];

		for byte in payload.as_bytes().iter() {
			if byte.is_ascii() {
				ascii_payload.push(*byte);
			} else {
				ascii_payload.push(0xfe);
			}
		}

		ascii_payload
	}
}

/// # The Ethernet Frame Header
///
/// The `EthernetFrameHeader` struct represents an Ethernet
/// frame header on L2 with all of its field nicely separated.
#[derive(Debug)]
pub struct EthernetHeader
{
	destination: [u8; ETH_MAC_ADDRESS_LENGTH],
	source:      [u8; ETH_MAC_ADDRESS_LENGTH],
	protocol:    [u8; ETH_PROTOCOL_ADDRESS_FIELD_LENGTH],
}

impl EthernetHeader
{
	pub fn get_destination_for_socket_address(
		&self,
	) -> [u8; ETH_MAC_ADDRESS_LENGTH + 2]
	{
		let mut sll_addr: [u8; 8] = [0x00; 8];

		for (i, field) in self.destination.iter().enumerate() {
			sll_addr[i] = *field;
		}

		sll_addr
	}
}
