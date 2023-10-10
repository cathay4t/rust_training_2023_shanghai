// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <cs_yaml.h>

int main(void) {
    int rc = EXIT_SUCCESS;
    uint32_t ret = CS_YAML_PASS;
    char *err_msg = NULL;
    char *log = NULL;

    ret = cs_yaml_report_time(&log, &err_msg);

    if (ret != CS_YAML_PASS) {
        printf("Error: %s\n", err_msg);
        rc = EXIT_FAILURE;
        goto out;
    }

    printf("LOG: %s\n", log);
 out:
    cs_yaml_cstring_free(err_msg);
    cs_yaml_cstring_free(log);
    exit(rc);
}
