
module SGS

include("deps.jl")
include("api.jl")

mutable struct SGSClient
    handle::Ptr{Cvoid}

    function SGSClient()
        handle = sgslib_client_new()

        if handle == C_NULL
            error("Failed to create SGSClient.")
        end

        new_client = new(handle)
        finalizer(destroy!, new_client)
        return new_client
    end
end

function destroy!(client::SGSClient)
    if client.handle != C_NULL
        sgslib_client_free(client.handle)
        client.handle = C_NULL
    end
    nothing
end

function __init__()
    check_deps()
end

function get_ultimo_valor_xml(client::SGSClient, serie_id::Int64) :: String
    out_xml_ref = Ref{Cstring}()
    error_code = sgslib_get_ultimo_valor_xml(client.handle, serie_id, out_xml_ref)

    if error_code != 0
        error(error_code)
    end

    result = unsafe_string(out_xml_ref[])
    sgslib_free_xml(out_xml_ref[])

    return result
end

end # module
