use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod bindings;
pub mod messages;
pub mod ports;
pub mod series;
pub mod services;
