
#=
int32_t sgslib_get_ultimo_valor_xml(const void *client_handle,
                                    int64_t serie_id,
                                    char **out_xml);
=#
function sgslib_get_ultimo_valor_xml(client_handle::Ptr{Cvoid}, serie_id::Int64, out_xml_ref::Ref{Cstring}) :: Int32
    ccall((:sgslib_get_ultimo_valor_xml, libsgs), Cint, (Ptr{Cvoid}, Int64, Ref{Cstring}), client_handle, serie_id, out_xml_ref)
end

# void sgslib_free_xml(char *xml);
function sgslib_free_xml(xml::Cstring)
    ccall((:sgslib_free_xml, libsgs), Cvoid, (Cstring,), xml)
end

# void *sgslib_client_new(void);
function sgslib_client_new()
    ccall((:sgslib_client_new, libsgs), Ptr{Cvoid}, ())
end

# void sgslib_client_free(void *client_handle);
function sgslib_client_free(client_handle::Ptr{Cvoid})
    ccall((:sgslib_client_free, libsgs), Cvoid, (Ptr{Cvoid},), client_handle)
end

#=
int32_t sgslib_get_valores_series_xml(const void *client_handle,
                                      const int64_t *series_vec,
                                      uintptr_t series_vec_len,
                                      int32_t from_gregorian,
                                      int32_t to_gregorian,
                                      char **out_xml);
=#
function sgslib_get_valores_series_xml(client_handle::Ptr{Cvoid}, series_vec_handle::Ptr{Int64}, series_vec_len::Csize_t, from_gregorian::Int32, to_gregorian::Int32, out_xml_ref::Ref{Cstring})
    ccall((:sgslib_get_valores_series_xml, libsgs), Int32, (Ptr{Cvoid}, Ptr{Int64}, Csize_t, Int32, Int32, Ref{Cstring}), client_handle, series_vec_handle, series_vec_len, from_gregorian, to_gregorian, out_xml_ref)
end
