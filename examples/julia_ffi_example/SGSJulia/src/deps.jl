
import Libdl

const libsgs = joinpath(@__DIR__, "..", "..", "..", "..", "target", "release", "libsgs.dylib")

function check_deps()
    global libsgs

    if !isfile(libsgs)
        error("$libsgs not found.")
    end

    if Libdl.dlopen_e(libsgs) == C_NULL
        error("$libsgs cannot be opened.")
    end
end
