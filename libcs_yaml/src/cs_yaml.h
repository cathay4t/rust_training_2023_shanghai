// SPDX-License-Identifier: Apache-2.0

#ifndef _LIBCS_YAML_H_
#define _LIBCS_YAML_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#define CS_YAML_VERSION_MAJOR        0
#define CS_YAML_VERSION_MINOR        1
#define CS_YAML_VERSION_MICRO        0

#define CS_YAML_VERSION              \
    ((CS_YAML_VERSION_MAJOR * 10000) + \
     (CS_YAML_VERSION_MINOR * 100) + \
     CS_YAML_VERSION_MICRO)

#define CS_YAML_PASS                 0
#define CS_YAML_FAIL                 1
#define CS_YAML_FAIL_NULL_POINTER    2

int cs_yaml_report_time(char ** log, char **err_msg);

void cs_yaml_cstring_free(char *cstring);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* End of _LIBCS_YAML_H_ */
