use super::messages;
use super::SoapFault;
use async_trait::async_trait;

#[async_trait]
pub trait FachadaWSSGS {
    async fn get_valores_series_xml(
        &self,
        get_valores_series_xml_request: messages::GetValoresSeriesXMLRequest,
    ) -> Result<messages::GetValoresSeriesXMLResponse, Option<SoapFault>>;

    async fn get_ultimo_valor_xml(
        &self,
        get_ultimo_valor_xml_request: messages::GetUltimoValorXMLRequest,
    ) -> Result<messages::GetUltimoValorXMLResponse, Option<SoapFault>>;
}
