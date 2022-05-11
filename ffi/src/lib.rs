use std::ffi::CString;
use std::os::raw::c_char;
use tokio::runtime::Runtime;

#[no_mangle]
pub unsafe extern "C" fn sgslib_get_ultimo_valor_xml(
    serie_id: i64,
    out_xml: *mut *mut c_char,
) -> i32 {
    Runtime::new()
        .unwrap()
        .block_on(async_sgslib_get_ultimo_valor_xml(serie_id, out_xml))
}

async unsafe fn async_sgslib_get_ultimo_valor_xml(serie_id: i64, out_xml: *mut *mut c_char) -> i32 {
    let client = sgslib::services::FachadaWSSGSService::new_client(Option::None);

    match sgslib::get_ultimo_valor_xml(&client, serie_id).await {
        Ok(str_xml) => {
            let cstring = CString::new(str_xml).unwrap();
            // transfers ownership to the caller

            *out_xml = cstring.into_raw();

            // return success code
            0
        }
        Err(_) => {
            // return error code
            1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sgslib_free_xml(xml: *mut c_char) {
    if !xml.is_null() {
        // retakes ownership and drop
        drop(CString::from_raw(xml));
    }
}
