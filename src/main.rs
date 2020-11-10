#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]

/// Provides all abstractions over frames, sockets, interaction, etc.
mod lib;

use lib::{
	defaults,
	frames,
	interaction,
	sockets,
};

/// Loops over the steps required for building and sending frames.
fn main()
{
	lib::interaction::log::init_console();

	let socket_descriptor = sockets::open::raw_socket(0x1234);

	loop {
		let information = defaults::Defaults::init();

		let ethernet_frame: frames::EthernetFrame =
			information.clone().into();
		let (frame_array, frame_length) = ethernet_frame.to_array();
		let socket_address = sockets::SocketAddress::construct_from(
			&ethernet_frame,
			&information,
		);

		sockets::send::packet(
			socket_descriptor,
			&frame_array,
			frame_length,
			socket_address,
		);

		if interaction::quit_if_asked(socket_descriptor) {
			break;
		}
	}
}
