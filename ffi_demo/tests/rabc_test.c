// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <rabc.h>

int main(void) {
    int rc = EXIT_SUCCESS;
    uint32_t ret = RABC_PASS;
    struct rabc_client *client = NULL;
    char *err_msg = NULL;
    char *name = "TestClient";
    char *name_ret = NULL;

    ret = rabc_client_new(&client, name, &err_msg);

    if (ret != RABC_PASS) {
        printf("Error: %s\n", err_msg);
        rc = EXIT_FAILURE;
        goto out;
    }

    name_ret = rabc_client_get_name(client);
    printf("Returned Name %s\n", name_ret);

 out:
    rabc_cstring_free(name_ret);
    rabc_cstring_free(err_msg);
    rabc_client_free(client);
    exit(rc);
}
