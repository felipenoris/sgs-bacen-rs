use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Item {
    #[yaserde(attribute, rename = "xmlns:xs")]
    schema: String,

    #[yaserde(attribute, rename = "xmlns:xsi")]
    instance: String,

    #[yaserde(attribute, rename = "xsi:type")]
    item_type: String,

    #[yaserde(text)]
    val: String,
}

impl Item {
    pub fn new(id: i64) -> Self {
        Item {
            schema: String::from("http://www.w3.org/2001/XMLSchema"),
            instance: String::from("http://www.w3.org/2001/XMLSchema-instance"),
            item_type: String::from("xs:long"),
            val: id.to_string(),
        }
    }

    pub fn val(&self) -> i64 {
        self.val.parse().unwrap()
    }
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct ItemList {
    #[yaserde(child, rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "getValoresSeriesXML",
    namespace = "ns0: http://publico.ws.casosdeuso.sgs.pec.bcb.gov.br",
    prefix = "ns0"
)]
pub struct GetValoresSeriesXMLRequest {
    #[yaserde(child)]
    pub in0: ItemList,

    #[yaserde(child)]
    pub in1: String,

    #[yaserde(child)]
    pub in2: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "getValoresSeriesXMLResponse",
    namespace = "ns1: http://publico.ws.casosdeuso.sgs.pec.bcb.gov.br",
    prefix = "ns1"
)]
pub struct GetValoresSeriesXMLResponse {
    #[yaserde(attribute, rename = "soapenv:encodingStyle")]
    pub encoding_style: String, // "http://schemas.xmlsoap.org/soap/encoding/"

    #[yaserde(child, rename = "getValoresSeriesXMLReturn")]
    pub get_valores_series_xml_return: GetValoresSeriesXMLReturn,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(rename = "getValoresSeriesXMLReturn")]
pub struct GetValoresSeriesXMLReturn {
    #[yaserde(attribute, rename = "xmlns:soapenc")]
    pub encoding: String, // "http://schemas.xmlsoap.org/soap/encoding/"

    #[yaserde(attribute, rename = "xsi:type")]
    pub val_type: String, // soapenc:string

    #[yaserde(text)]
    pub val: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "getUltimoValorXMLRequest"
    namespace = "ns0: http://publico.ws.casosdeuso.sgs.pec.bcb.gov.br",
    prefix = "ns0"
)]
pub struct GetUltimoValorXMLRequest {
    #[yaserde(child)]
    pub in0: Item,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "getUltimoValorXMLResponse",
    namespace = "ns1: http://publico.ws.casosdeuso.sgs.pec.bcb.gov.br",
    prefix = "ns1"
)]
pub struct GetUltimoValorXMLResponse {
    #[yaserde(child, rename = "getUltimoValorXMLReturn")]
    pub get_ultimo_valor_xml_return: String,
}

// cargo test -- --nocapture
#[test]
fn serialize_get_valores_series_xml() {
    let item1 = Item::new(1);
    let item2 = Item::new(2);
    let vec_items = vec![item1, item2];
    let item_list = ItemList { items: vec_items };

    let message = GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: String::from("31/03/2022"),
        in2: String::from("31/03/2023"),
    };

    // Pretty Printed XML
    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    let serialized = yaserde::ser::to_string_with_config(&message, &yaserde_cfg).unwrap();
    println!("{:?}", serialized);

    let return_message = GetValoresSeriesXMLReturn {
        encoding: String::from("http://schemas.xmlsoap.org/soap/encoding/"),
        val_type: String::from("soapenc:string"),
        val: String::from("<?xml version='1.0'> <ITEM> 10 </ITEM>"),
    };

    let return_response = GetValoresSeriesXMLResponse {
        encoding_style: String::from("http://schemas.xmlsoap.org/soap/encoding/"),
        get_valores_series_xml_return: return_message,
    };

    let serialized = yaserde::ser::to_string_with_config(&return_response, &yaserde_cfg).unwrap();
    println!("{:?}", serialized);
}
