
module SGS

using Dates

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

function get_valores_series_xml(client::SGSClient, series_vec::Vector{Int64}, from::Date, to::Date)
    out_xml_ref = Ref{Cstring}()

    series_vec_len = Csize_t(length(series_vec))
    series_vec_handle = pointer(series_vec)
    from_gregorian = Int32(Dates.value(from))
    to_gregorian = Int32(Dates.value(to))
    error_code = sgslib_get_valores_series_xml(client.handle, series_vec_handle, series_vec_len, from_gregorian, to_gregorian, out_xml_ref)

    if error_code != 0
        error(error_code)
    end

    result = unsafe_string(out_xml_ref[])
    sgslib_free_xml(out_xml_ref[])

    return result
end

end # module
