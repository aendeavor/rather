use super::{
	defaults,
	frames,
	ETH_MAC_ADDRESS_LENGTH,
	PF_PACKET,
};

/// # The C Struct `sockaddr_ll`
///
/// This struct represents the `struct sockaddr_l`
/// from the C kernel header `include/uapi/linux/if_packet.h`.
///
/// ## Creation
///
/// The creation of this struct is **not** straightforward.
/// Therefore, this struct is not public and only worked on
/// via defined methods.
/// 
/// ## The C Variant
/// 
/// The `sockaddr_ll` struct in C looks like this:
/// 
/// ``` C
/// struct sockaddr_ll {
///     unsigned short  sll_family;
///     __be16          sll_protocol;
///     int             sll_ifindex;
///     unsigned short  sll_hatype;
///     unsigned char   sll_pkttype;
///     unsigned char   sll_halen;
///     unsigned char   sll_addr[8];
///};
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct SocketAddress
{
	sll_family:   u16,
	sll_protocol: u16,
	sll_ifindex:  u32, // should be i32 but casting issues (valid) warning
	sll_hatype:   u16,
	sll_pkttype:  u8,
	sll_halen:    u8,
	sll_addr:     [u8; 8],
}

impl SocketAddress
{
	pub fn construct_from(
		frame: &frames::EthernetFrame,
		information: &defaults::Defaults
	) -> Self
	{
		#[allow(clippy::cast_possible_truncation)]
		Self {
			sll_family:   PF_PACKET,
			sll_protocol: 0,  // useless
			sll_ifindex:  information.interface.index,
			sll_hatype:   0,  // useless
			sll_pkttype:  0,  // useless
			sll_halen:    ETH_MAC_ADDRESS_LENGTH as u8,
			sll_addr:     frame.header.get_destination_for_socket_address(),
		}
	}
}

pub mod send
{
	use std::os::raw;

	/// Wraps the unsafe extern "C" method and provides some
	/// defaults.
	pub fn packet(
		socket_descriptor: i32,
		frame: &[u8],
		frame_length: usize,
		socket_address: super::SocketAddress,
	) -> isize
	{
		unsafe {
			send_to(
				socket_descriptor,
				frame.as_ptr() as *const raw::c_char,
				frame_length,
				0,
				socket_address,
			)
		}
	}

	extern "C" {
		/// Wraps the `sendto()` function from
		/// `sys/socket.h`. This function is provided
		/// externally in `components/bindings.c`.
		fn send_to(
			socket_descriptor: raw::c_int,
			frame: *const raw::c_char,
			frame_length: usize,
			flags: raw::c_int,
			socket_address: super::SocketAddress,
		) -> isize;
	}
}

pub mod open
{
	use std::io::{
		self,
		ErrorKind,
	};
	use super::super::{
		AF_PACKET,
		SOCK_RAW,
	};

	/// Opens a raw socket or panics in case of failure.
	pub fn raw_socket(protocol: i32) -> i32
	{
		unsafe {
			match libc::socket(AF_PACKET, SOCK_RAW, protocol.to_be()) {
				-1 | 0 => {
					let last_error = io::Error::last_os_error();
					match last_error.kind() {
						ErrorKind::PermissionDenied => eprintln!(
							"Permission denied. Please execute this \
							binary with privileges."
						),
						_ => eprintln!("Error: {:?}", last_error),
					};
					std::process::exit(1);
				},
				fd => fd,
			}
		}
	}
}

pub mod close
{
	/// Closes a socket described by its descriptor.
	pub fn raw_socket(socket_descriptor: i32)
	{
		unsafe {
			libc::close(socket_descriptor);
		}
	}
}
