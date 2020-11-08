use super::{
	defaults,
	sockets,
	ETH_MAC_ADDRESS_LENGTH,
};


/// # Construct Information from User Input
///
/// Constructs the correct information when the user
pub fn construct_options(information: &mut defaults::Defaults)
{
	if let Some(interface) = get_input("Interface: ") {
		let interfaces = pnet::datalink::interfaces();
		let interfaces = interfaces
			.iter()
			.filter(|network_interface| {
				network_interface.name == interface
			})
			.collect::<Vec<&pnet::datalink::NetworkInterface>>();

		if interfaces.is_empty() {
			// TODO handle this properly
			eprintln!("There is no such interface. Proceeding.")
		} else {
			information.source = defaults::translate_mac_to_array(
				interfaces[0].mac.unwrap(),
			);
			information.interface = interfaces[0].to_owned();
		}
	}

	if let Some(destination) = get_input("Destination MAC address: ") {
		for (i, oktett) in destination.split(':').enumerate() {
			information.destination[i] =
				u8::from_str_radix(oktett, 16).unwrap();

			if i == ETH_MAC_ADDRESS_LENGTH {
				break;
			}
		}
	}

	if let Some(data) = get_input("Payload: ") {
		information.set_data(data);
	}
}

/// # Read User Input
///
/// Reads user input, sanitizes it and provides it back
/// to the caller.
pub fn get_input(question: &str) -> Option<String>
{
	use std::io::{
		stdin,
		stdout,
		Write,
	};

	let mut input = String::new();

	loop {
		print!("{}", question);
		let _ = stdout().flush();

		match stdin().read_line(&mut input) {
			Ok(_) => break,
			Err(_) => eprintln!("Entered input is invalid."),
		}
	}

	if let Some('\n') = input.chars().next_back() {
		input.pop();
	}

	if let Some('\r') = input.chars().next_back() {
		input.pop();
	}

	if input == "" {
		None
	} else {
		Some(input)
	}
}

pub fn quit_if_asked(socket_descriptor: i32) -> bool
{
	return match get_input("\nWould you like to go on? [Y/n] ")
	{
		Some(answer) => {
			match answer.as_str() {
				"n" | "N" | "no" | "No" => {
					sockets::close::raw_socket(socket_descriptor);
					true
				},
				_ => false,
			}
		},
		None => false
	}
}

pub mod log
{
	/// Just prints some information during startup.
	pub fn init_console()
	{
		println!("");
		print("RATHER :: v0.1.4");
		print("Construct Ethernet packages and send them.");
	}

	fn print(string: &str)
	{
		let mut underscores: String = String::new();
		for _ in 0..string.len() {
			underscores += "\u{2013}";
		}

		println!("{}\n{}\n", string, underscores);
	}
}
