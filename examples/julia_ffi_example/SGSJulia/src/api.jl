
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
