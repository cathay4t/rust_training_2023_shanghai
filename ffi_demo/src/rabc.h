// SPDX-License-Identifier: Apache-2.0

#ifndef _LIBRABC_H_
#define _LIBRABC_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#define RABC_VERSION_MAJOR        0
#define RABC_VERSION_MINOR        1
#define RABC_VERSION_MICRO        0

#define RABC_VERSION              \
    ((RABC_VERSION_MAJOR * 10000) + \
     (RABC_VERSION_MINOR * 100) + \
     RABC_VERSION_MICRO)

#define RABC_PASS                 0
#define RABC_FAIL                 1
#define RABC_FAIL_NULL_POINTER    2

struct rabc_client;

int rabc_client_new(struct rabc_client **client, char *name, char **err_msg);

// Returned memory should be freed by rabc_cstring_free().
char * rabc_client_get_name(struct rabc_client *client);

void rabc_client_free(struct rabc_client *client);

void rabc_cstring_free(char *cstring);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _LIBRABC_H_ */
