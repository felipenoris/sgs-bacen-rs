use super::messages;
use super::ports;
use super::{Header, SoapFault, SoapResponse, SOAP_ENCODING};
use async_trait::async_trait;
use yaserde::YaSerialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

impl Default for FachadaWSSGSSoapBinding {
    fn default() -> Self {
        FachadaWSSGSSoapBinding {
            client: reqwest::Client::new(),
            url: "https://www3.bcb.gov.br/wssgs/services/FachadaWSSGS".to_string(),
            credentials: Option::None,
        }
    }
}
impl FachadaWSSGSSoapBinding {
    pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
        FachadaWSSGSSoapBinding {
            client: reqwest::Client::new(),
            url: url.to_string(),
            credentials,
        }
    }
}
pub struct FachadaWSSGSSoapBinding {
    client: reqwest::Client,
    url: String,
    credentials: Option<(String, String)>,
}

impl FachadaWSSGSSoapBinding {
    async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
        let body = yaserde::ser::to_string(request).expect("failed to generate xml");
        //println!("SOAP Request: {}", body);
        let mut req = self
            .client
            .post(&self.url)
            .body(body)
            .header("Content-Type", "text/xml")
            .header("Soapaction", action);
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(
                credentials.0.to_string(),
                Option::Some(credentials.1.to_string()),
            );
        }
        let res = req.send().await?;
        let status = res.status();
        //println!("SOAP Status: {}", status);
        let txt = res.text().await.unwrap_or_default();
        //println!("SOAP Response: {}", txt);
        Ok((status, txt))
    }
}

#[async_trait]
impl ports::FachadaWSSGS for FachadaWSSGSSoapBinding {
    async fn get_valores_series_xml(
        &self,
        get_valores_series_xml_request: messages::GetValoresSeriesXMLRequest,
    ) -> Result<messages::GetValoresSeriesXMLResponse, Option<SoapFault>> {
        let __request =
            GetValoresSeriesXMLRequestSoapEnvelope::new(SoapGetValoresSeriesXMLRequest {
                body: get_valores_series_xml_request,
                xmlns: Option::Some(
                    "https://www3.bcb.gov.br/wssgs/services/FachadaWSSGS".to_string(),
                ),
            });

        let (status, response) = self
            .send_soap_request(&__request, "")
            .await
            .map_err(|err| {
                println!("Failed to send SOAP request: {:?}", err);
                None
            })?;

        let r: GetValoresSeriesXMLResponseSoapEnvelope =
            yaserde::de::from_str(&response).map_err(|err| {
                println!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }
    }
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct SoapGetValoresSeriesXMLRequest {
    #[yaserde(rename = "getValoresSeriesXML", default)]
    pub body: messages::GetValoresSeriesXMLRequest,
    #[yaserde(attribute)]
    pub xmlns: Option<String>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Envelope",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct GetValoresSeriesXMLRequestSoapEnvelope {
    #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
    pub encoding_style: String,
    #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
    pub tnsattr: Option<String>,
    #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
    pub urnattr: Option<String>,
    #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
    pub xsiattr: Option<String>,
    #[yaserde(rename = "Header", prefix = "soapenv")]
    pub header: Option<Header>,
    #[yaserde(rename = "Body", prefix = "soapenv")]
    pub body: SoapGetValoresSeriesXMLRequest,
}

impl GetValoresSeriesXMLRequestSoapEnvelope {
    pub fn new(body: SoapGetValoresSeriesXMLRequest) -> Self {
        GetValoresSeriesXMLRequestSoapEnvelope {
            encoding_style: SOAP_ENCODING.to_string(),
            tnsattr: Option::Some(
                "https://www3.bcb.gov.br/wssgs/services/FachadaWSSGS".to_string(),
            ),
            body,
            urnattr: None,
            xsiattr: None,
            header: None,
        }
    }
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct SoapGetValoresSeriesXMLResponse {
    #[yaserde(rename = "getValoresSeriesXMLResponse", default)]
    pub body: messages::GetValoresSeriesXMLResponse,
    #[yaserde(rename = "Fault", default)]
    pub fault: Option<SoapFault>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Envelope",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct GetValoresSeriesXMLResponseSoapEnvelope {
    #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
    pub encoding_style: String,
    #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
    pub tnsattr: Option<String>,
    #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
    pub urnattr: Option<String>,
    #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
    pub xsiattr: Option<String>,
    #[yaserde(rename = "Header", prefix = "soapenv")]
    pub header: Option<Header>,
    #[yaserde(rename = "Body", prefix = "soapenv")]
    pub body: SoapGetValoresSeriesXMLResponse,
}

/*
impl GetValoresSeriesXMLResponseSoapEnvelope {
    pub fn new(body: SoapGetValoresSeriesXMLResponse) -> Self {
        GetValoresSeriesXMLResponseSoapEnvelope {
            encoding_style: SOAP_ENCODING.to_string(),
            tnsattr: Option::Some(
                "https://www3.bcb.gov.br/wssgs/services/FachadaWSSGS".to_string(),
            ),
            body,
            urnattr: None,
            xsiattr: None,
            header: None,
        }
    }
}
*/
