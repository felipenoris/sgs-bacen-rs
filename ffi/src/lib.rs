use sgslib::bindings::FachadaWSSGSSoapBinding;
use std::ffi::CString;
use std::os::raw::c_char;
use tokio::runtime::Runtime;

#[no_mangle]
pub unsafe extern "C" fn sgslib_get_ultimo_valor_xml(
    client_handle: *const FachadaWSSGSSoapBinding,
    serie_id: i64,
    out_xml: *mut *mut c_char,
) -> i32 {
    Runtime::new()
        .unwrap()
        .block_on(async_sgslib_get_ultimo_valor_xml(
            client_handle,
            serie_id,
            out_xml,
        ))
}

#[no_mangle]
pub extern "C" fn sgslib_client_new() -> *mut FachadaWSSGSSoapBinding {
    let client = sgslib::services::FachadaWSSGSService::new_client(Option::None);
    Box::into_raw(Box::new(client))
}

#[no_mangle]
pub unsafe extern "C" fn sgslib_client_free(client_handle: *mut FachadaWSSGSSoapBinding) {
    if !client_handle.is_null() {
        Box::from_raw(client_handle);
    }
}

unsafe fn ptr_into_client_ref<'a>(client_handle_ref: &'a *const FachadaWSSGSSoapBinding) -> &'a FachadaWSSGSSoapBinding {
    assert!(!client_handle_ref.is_null());
    &**client_handle_ref
}

async fn async_sgslib_get_ultimo_valor_xml(
    client_handle: *const FachadaWSSGSSoapBinding,
    serie_id: i64,
    out_xml: *mut *mut c_char,
) -> i32 {

    let client = unsafe {
        ptr_into_client_ref(&client_handle)
    };

    match sgslib::get_ultimo_valor_xml(client, serie_id).await {
        Ok(str_xml) => {
            let cstring = CString::new(str_xml).unwrap();
            // transfers ownership to the caller

            unsafe {
                *out_xml = cstring.into_raw();
            }

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
