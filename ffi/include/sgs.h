#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t sgslib_get_ultimo_valor_xml(const void *client_handle,
                                    int64_t serie_id,
                                    char **out_xml);

void *sgslib_client_new(void);

void sgslib_client_free(void *client_handle);

void sgslib_free_xml(char *xml);

int32_t sgslib_get_valores_series_xml(const void *client_handle,
                                      const int64_t *series_vec,
                                      uintptr_t series_vec_len,
                                      int32_t from_gregorian,
                                      int32_t to_gregorian,
                                      char **out_xml);
