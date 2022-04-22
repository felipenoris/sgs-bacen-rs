use crate::sgs::ports::FachadaWSSGS;

mod sgs;

#[tokio::main]
async fn main() {
    let sgs_client = sgs::services::FachadaWSSGSService::new_client(Option::None);

    let item1 = sgs::messages::Item::new(1);
    assert!(item1.val() == 1);
    let vec_items = vec![item1];
    let item_list = sgs::messages::ItemList { items: vec_items };

    let request = sgs::messages::GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: String::from("31/03/2022"),
        in2: String::from("31/03/2023"),
    };

    let response = sgs_client.get_valores_series_xml(request).await;

    let series_xml = response.unwrap().get_valores_series_xml_return.val;
    println!("XML Result:\n{:?}", series_xml);
    let series: sgs::series::Series = yaserde::de::from_str(&series_xml).unwrap();
    println!("{:?}", series);
}
