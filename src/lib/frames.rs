use super::{
	super::defaults,
	ETH_PAYLOAD_LENGTH,
	ETH_MAC_ADDRESS_LENGTH,
	ETH_PROTOCOL_ADDRESS_FIELD_LENGTH,
	ETH_FRAME_LENGTH,
	ETH_HEADER_LENGTH
};

/// # The Ethernet Frame
///
/// The `EthernetFrame` struct represents an Ethernet
/// frame on L2 with all of its field nicely separated.
///
/// ## Byte Order
///
/// The network byteorder is used, i.e. Big Endian.
#[repr(C)]
#[derive(Debug)]
pub struct EthernetFrame
{
	pub header:  EthernetHeader,
	payload: [u8; ETH_PAYLOAD_LENGTH],
}

impl From<defaults::Defaults> for EthernetFrame
{
	fn from(information: defaults::Defaults) -> Self
	{
		
		let ethernet_header = EthernetHeader {
			destination: information.destination,
			source: information.source,
			protocol: information.protocol
		};

		let mut ethernet_frame = Self {
			header: ethernet_header,
			payload: [0x00; ETH_PAYLOAD_LENGTH]
		};

		ethernet_frame.serialize_string_payload_to_ascii(&information.data);
		ethernet_frame
	}
}

impl EthernetFrame
{
	// TODO PRINTS STATIC ETH_FRAME_LENGTH BYTE LONG FRAMES EVERY TIME
	pub fn to_array(&self) -> (Box<[u8]>, usize)
	{
		let mut serialized_array: [u8; ETH_FRAME_LENGTH] = [0x00; ETH_FRAME_LENGTH];
		
		for (i, field) in self.header.destination.iter().enumerate()
		{
			serialized_array[i] = *field;
		}

		for (i, field) in self.header.source.iter().enumerate()
		{
			serialized_array[i + ETH_MAC_ADDRESS_LENGTH] = *field;
		}

		for (i, field) in self.header.protocol.iter().enumerate()
		{
			serialized_array[i + ETH_MAC_ADDRESS_LENGTH * 2] = *field;
		}

		for (i, field) in self.payload.iter().enumerate()
		{
			serialized_array[i + ETH_HEADER_LENGTH] = *field;
		}
		
		(Box::new(serialized_array), ETH_FRAME_LENGTH)
	}

	fn serialize_string_payload_to_ascii(&mut self, payload: &str)
	{
		for (i, byte) in payload.as_bytes().iter().enumerate() {
			if byte.is_ascii() {
				self.payload[i] = *byte
			} else {
				self.payload[i] = 0xfe
			}
		}
	}
}

/// # The Ethernet Frame Header
///
/// The `EthernetFrameHeader` struct represents an Ethernet
/// frame header on L2 with all of its field nicely separated.
#[repr(C)]
#[derive(Debug)]
pub struct EthernetHeader
{
	destination: [u8; ETH_MAC_ADDRESS_LENGTH],
	source:      [u8; ETH_MAC_ADDRESS_LENGTH],
	protocol:    [u8; ETH_PROTOCOL_ADDRESS_FIELD_LENGTH],
}

impl EthernetHeader
{
	pub fn get_destination_for_socket_address(&self) -> [u8; ETH_MAC_ADDRESS_LENGTH + 2]
	{
		let mut sll_addr: [u8; 8] = [0x00; 8];
		
		for (i, field) in self.destination.iter().enumerate()
		{
			sll_addr[i] = *field;
		}
		
		sll_addr
	}
}
