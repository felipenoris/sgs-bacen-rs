use super::bindings;

pub struct FachadaWSSGSService {}
impl FachadaWSSGSService {
    pub fn new_client(credentials: Option<(String, String)>) -> bindings::FachadaWSSGSSoapBinding {
        bindings::FachadaWSSGSSoapBinding::new(
            "https://www3.bcb.gov.br/wssgs/services/FachadaWSSGS",
            credentials,
        )
    }
}
