module SGSJulia

include("deps.jl")
include("api.jl")

function __init__()
    check_deps()
end

function get_ultimo_valor_xml(serie_id::Int64) :: String
    out_xml_ref = Ref{Cstring}()
    error_code = sgslib_get_ultimo_valor_xml(serie_id, out_xml_ref)

    if error_code != 0
        error(error_code)
    end

    result = unsafe_string(out_xml_ref[])
    sgslib_free_xml(out_xml_ref[])

    return result
end

end # module
