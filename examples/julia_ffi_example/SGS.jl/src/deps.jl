
import Libdl

const LIB_NAME = "libsgs"

function libsgs_basename() :: String
    if Sys.isapple()
        "$LIB_NAME.dylib"
    elseif Sys.islinux()
        "$LIB_NAME.so"
    elseif Sys.iswindows()
        "$LIB_NAME.dll"
    else
        error("Not supported: $(Sys.KERNEL)")
    end
end

function resolve_sgslib_filepath()
    release_dir = joinpath(@__DIR__, "..", "..", "..", "..", "target", "release")
    return joinpath(release_dir, libsgs_basename())
end

const libsgs = resolve_sgslib_filepath()

function check_deps()
    global libsgs

    if !isfile(libsgs)
        error("$libsgs not found. Run `cargo build --release` to build the package.")
    end

    if Libdl.dlopen_e(libsgs) == C_NULL
        error("$libsgs cannot be opened.")
    end
end
