use clap::{Parser, Subcommand};
use sgslib::messages::Item;
use sgslib::ports::FachadaWSSGS;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Series {
        #[clap(short, long)]
        list: String,

        #[clap(short, long, value_name = "dd/mm/yyyy")]
        from: String,

        #[clap(short, long, value_name = "dd/mm/yyyy")]
        to: String,
    },

    LastValue {
        id: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Series { list, from, to } => {
            execute_get_series(list, from, to).await;
        }
        Commands::LastValue { id } => {
            execute_get_ultimo_valor(&id).await;
        }
    }
}

async fn execute_get_series(list: String, from: String, to: String) {
    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);
    let mut vec_items: Vec<Item> = Vec::new();

    for id in list.split(',') {
        vec_items.push(Item::new(id.parse().unwrap()));
    }

    let item_list = sgslib::messages::ItemList { items: vec_items };

    let request = sgslib::messages::GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: from,
        in2: to,
    };

    let response = sgs_client.get_valores_series_xml(request).await;

    let series_xml = response.unwrap().get_valores_series_xml_return.val;
    println!("{}", series_xml);
}

async fn execute_get_ultimo_valor(id: &str) {
    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);

    let ultimo_valor = sgs_client
        .get_ultimo_valor_xml(sgslib::messages::GetUltimoValorXMLRequest {
            in0: sgslib::messages::Item::new(id.parse().unwrap()),
        })
        .await;
    let ultimo_valor = ultimo_valor.unwrap().get_ultimo_valor_xml_return;
    println!("{}", ultimo_valor);
}
