use super::{
	ETH_MAC_ADDRESS_LENGTH,
	ETH_PROTOCOL_ADDRESS_FIELD_LENGTH,
};

use pnet::datalink::{
	MacAddr,
	NetworkInterface,
};

/// Provides a data strcuture to create and store
/// all settings
#[derive(Debug, Clone)]
pub struct Defaults
{
	pub interface:   NetworkInterface,
	pub protocol:    [u8; ETH_PROTOCOL_ADDRESS_FIELD_LENGTH],
	pub source:      [u8; ETH_MAC_ADDRESS_LENGTH],
	pub destination: [u8; ETH_MAC_ADDRESS_LENGTH],

	pub data:    String,
	data_length: usize,
}

impl std::default::Default for Defaults
{
	fn default() -> Self
	{
		// This is probably `lo`, the loopback interface
		let interface = pnet::datalink::interfaces()[1].to_owned();

		let protocol: [u8; ETH_PROTOCOL_ADDRESS_FIELD_LENGTH] =
			[0x12, 0x34];

		let destination: [u8; ETH_MAC_ADDRESS_LENGTH] = match interface.mac
		{
			Some(mac) => translate_mac_to_array(mac),
			None => [0; ETH_MAC_ADDRESS_LENGTH],
		};
		let source: [u8; ETH_MAC_ADDRESS_LENGTH] = destination;

		Self {
			destination,
			source,

			interface,
			protocol,

			data: String::from("Hello, World!"),
			data_length: 13,
		}
	}
}

impl Defaults
{
	pub fn init() -> Self
	{
		let mut information = Self::default();
		super::interaction::construct_options(&mut information);
		information
	}

	pub fn set_data(&mut self, data: String)
	{
		self.data_length = data.len();
		self.data = data;
	}
}

/// Translates a `pnet::datalink::MacAddr` to an array `[u8;
/// ETH_MAC_ADDRESS_LENGTH]`.
pub const fn translate_mac_to_array(
	mac: MacAddr,
) -> [u8; ETH_MAC_ADDRESS_LENGTH]
{
	let mut mac_array: [u8; ETH_MAC_ADDRESS_LENGTH] =
		[0; ETH_MAC_ADDRESS_LENGTH];

	mac_array[0] = mac.0;
	mac_array[1] = mac.1;
	mac_array[2] = mac.2;
	mac_array[3] = mac.3;
	mac_array[4] = mac.4;
	mac_array[5] = mac.5;

	mac_array
}
