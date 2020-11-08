#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h> /* Must precede if*.h */
#include <linux/if.h>
#include <linux/if_ether.h>
#include <linux/if_packet.h>
#include <sys/ioctl.h>
#include <arpa/inet.h>

#include "debug.h"

union ethframe
{
	struct
	{
		struct ethhdr header;
		unsigned char data[ETH_DATA_LEN];
	} field;

	unsigned char buffer[ETH_FRAME_LEN];
};

int main()
{
	char *iface = "enp3s0";
	unsigned char dest[ETH_ALEN] = {0xd4, 0x3d, 0x7e, 0xbd, 0xf8, 0x1c};
	unsigned short proto = 0x1234;
	unsigned char data[14] = "Hello, World!";
	unsigned short data_len = 14;

	int socket_descriptor;
	if ((socket_descriptor = socket(AF_PACKET, SOCK_RAW, htons(proto))) < 0)
	{
		printf("Error: could not open socket\n");
		return -1;
	}

	struct ifreq buffer;
	int interface_index;

	memset(&buffer, 0x00, sizeof(buffer));
	strncpy(buffer.ifr_name, iface, IFNAMSIZ);
	if (ioctl(socket_descriptor, SIOCGIFINDEX, &buffer) < 0)
	{
		printf("Error: could not get interface index\n");
		close(socket_descriptor);
		return -1;
	}
	interface_index = buffer.ifr_ifindex;

	unsigned char source[ETH_ALEN];
	if (ioctl(socket_descriptor, SIOCGIFHWADDR, &buffer) < 0)
	{
		printf("Error: could not get interface address\n");
		close(socket_descriptor);
		return -1;
	}
	memcpy((void *)source, (void *)(buffer.ifr_hwaddr.sa_data), ETH_ALEN);

	union ethframe frame;
	memcpy(frame.field.header.h_dest, dest, ETH_ALEN);
	memcpy(frame.field.header.h_source, source, ETH_ALEN);
	frame.field.header.h_proto = htons(proto);
	memcpy(frame.field.data, data, data_len);

	struct sockaddr_ll saddr_ll;
	memset((void *)&saddr_ll, 0, sizeof(saddr_ll));
	saddr_ll.sll_family = PF_PACKET;
	saddr_ll.sll_ifindex = interface_index;
	saddr_ll.sll_halen = ETH_ALEN;
	memcpy((void *)(saddr_ll.sll_addr), (void *)dest, ETH_ALEN);

    debug_sockaddr_ll(&saddr_ll);
	debug_sockaddr((struct sockaddr *)&saddr_ll);

	printf("DEBUG :: Buffer = %s\n\n", frame.buffer);

	if (sendto(socket_descriptor, frame.buffer, data_len + ETH_HLEN, 0,
			   (struct sockaddr *)&saddr_ll, sizeof(saddr_ll)) > 0)
		printf("Success!\n");
	else
		printf("Error, could not send\n");

	close(socket_descriptor);

	return 0;
}
