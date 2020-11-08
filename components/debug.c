#include "debug.h"
#include <stdio.h>

void debug_sockaddr_ll(struct sockaddr_ll *socket_adress)
{
	printf("This is from C!\nDEBUG  :: START\n");
	printf("STRUCT :: sockaddr_ll with size %li\n\n  ", sizeof(*socket_adress));

	printf("%hu ", socket_adress->sll_protocol);
	printf("%i ", socket_adress->sll_ifindex);
	printf("%hu ", socket_adress->sll_hatype);
	printf("%d ", socket_adress->sll_halen);
	for (int i = 0; i < 8; ++i)
		printf("%d ", socket_adress->sll_addr[i]);

	printf("\n\nDEBUG  :: END\n\n");
}

void debug_sockaddr(struct sockaddr *socket_adress)
{
	printf("This is from C!\nDEBUG  :: START\n");
	printf("STRUCT :: sockaddr with size %li\n\n  ", sizeof(*socket_adress));

	for (int i = 0; i < 14; ++i)
		printf("%d ", socket_adress->sa_data[i]);
	
	printf("\n  extended (out-of-bounds): ");
	for (int i = 14; i < 18; ++i)
		printf("%d ", socket_adress->sa_data[i]);

	printf("\n\nDEBUG  :: END\n\n");
}
