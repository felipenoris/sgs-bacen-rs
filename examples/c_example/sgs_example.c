
#include <stdio.h>
#include <sgs.h>
#include <stdint.h>

int main() {
    void *client = sgslib_client_new();

    char *out_xml;

    sgslib_get_ultimo_valor_xml(client, 12, &out_xml);
    printf("%s\n", out_xml);
    sgslib_free_xml(out_xml);

    int64_t series[]={1,12};
    sgslib_get_valores_series_xml(client, series, 2, 738276, 738280, &out_xml);
    printf("%s\n", out_xml);
    sgslib_free_xml(out_xml);

    sgslib_client_free(client);
}
