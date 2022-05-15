
# int32_t sgslib_get_ultimo_valor_xml(int64_t serie_id, char **out_xml);
function sgslib_get_ultimo_valor_xml(serie_id::Int64, out_xml_ref::Ref{Cstring}) :: Int32
	ccall((:sgslib_get_ultimo_valor_xml, libsgs), Cint, (Int64, Ref{Cstring}), serie_id, out_xml_ref)
end

# void sgslib_free_xml(char *xml);
function sgslib_free_xml(xml::Cstring)
	ccall((:sgslib_free_xml, libsgs), Cvoid, (Cstring,), xml)
end
