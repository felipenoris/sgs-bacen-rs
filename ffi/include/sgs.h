#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t sgslib_get_ultimo_valor_xml(int64_t serie_id, char **out_xml);

void sgslib_free_xml(char *xml);
