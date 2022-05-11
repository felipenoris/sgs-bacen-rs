use chrono::NaiveDate;
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

fn into_sgs_date_format(date: NaiveDate) -> String {
    date.format("%d/%m/%Y").to_string()
}

#[test]
fn test_into_sgs_date_format() {
    let dt = NaiveDate::from_ymd(2015, 3, 1);
    assert!(into_sgs_date_format(dt) == "01/03/2015");
}

fn into_items_list(series: &[i64]) -> messages::ItemList {
    let mut vec_items: Vec<messages::Item> = Vec::with_capacity(series.len());

    for id in series {
        vec_items.push(messages::Item::new(*id));
    }

    messages::ItemList { items: vec_items }
}

pub async fn get_valores_series_xml(
    client: &dyn ports::FachadaWSSGS,
    series: &[i64],
    from: NaiveDate,
    to: NaiveDate,
) -> Result<String, Option<SoapFault>> {
    let item_list = into_items_list(series);

    let request = messages::GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: into_sgs_date_format(from),
        in2: into_sgs_date_format(to),
    };

    let response = client.get_valores_series_xml(request).await;

    match response {
        Ok(response) => {
            let series_xml = response.get_valores_series_xml_return.val;
            Ok(series_xml)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ultimo_valor_xml(
    client: &dyn ports::FachadaWSSGS,
    serie_id: i64,
) -> Result<String, Option<SoapFault>> {
    let ultimo_valor = client
        .get_ultimo_valor_xml(messages::GetUltimoValorXMLRequest {
            in0: messages::Item::new(serie_id),
        })
        .await;

    match ultimo_valor {
        Ok(ultimo_valor) => Ok(ultimo_valor.get_ultimo_valor_xml_return),
        Err(err) => Err(err),
    }
}
