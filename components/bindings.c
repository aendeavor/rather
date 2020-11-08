#include "bindings.h"
#include "debug.h"

#include <stdio.h>
void internal_debug(struct sockaddr_ll * socket_adress, const char * frame)
{
    printf("\n");
	debug_sockaddr_ll(socket_adress);
    debug_sockaddr((struct sockaddr *) socket_adress);
    printf("DEBUG :: Buffer = %s", frame);

	fflush(stdout);
}

long send_to(
	int socket_descriptor,
	const char *frame,
	size_t frame_length,
	int flags,
	struct sockaddr_ll socket_adress)
{
    // internal_debug(&socket_adress, frame);

	return sendto(
		socket_descriptor,
        frame,
        frame_length,
		flags,
		(struct sockaddr *)&socket_adress,
		sizeof(socket_adress));
}
