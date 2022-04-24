use sgslib::ports::FachadaWSSGS;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);

    let ultimo_valor = sgs_client
        .get_ultimo_valor_xml(sgslib::messages::GetUltimoValorXMLRequest {
            in0: sgslib::messages::Item::new(1),
        })
        .await;
    let ultimo_valor = ultimo_valor.unwrap().get_ultimo_valor_xml_return;
    println!("Ultimo valor:\n{}", ultimo_valor);

    let item1 = sgslib::messages::Item::new(1);
    assert!(item1.val() == 1);
    let vec_items = vec![item1];
    let item_list = sgslib::messages::ItemList { items: vec_items };

    let request = sgslib::messages::GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: String::from("31/03/2022"),
        in2: String::from("31/03/2023"),
    };

    let response = sgs_client.get_valores_series_xml(request).await;

    let series_xml = response.unwrap().get_valores_series_xml_return.val;
    println!("XML Result:\n{}", series_xml);
    let series = sgslib::series::Series::from_str(&series_xml).unwrap();
    println!("{:?}", series);
}
