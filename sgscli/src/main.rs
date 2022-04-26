use clap::{Parser, Subcommand};
use sgslib::messages::Item;
use sgslib::ports::FachadaWSSGS;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// query values of one or more series inside a given period
    Series {
        /// list of identifiers
        list: Vec<String>,

        /// initial date
        #[clap(short, long, value_name = "date")]
        from: String,

        /// end date
        #[clap(short, long, value_name = "date")]
        to: String,
    },

    /// query the last value of a time serie
    LastValue {
        /// serie identifier
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

#[derive(Debug, Clone)]
struct InvalidDateStringFormat {
    date_str: String,
}

fn exit_with_error(err: InvalidDateStringFormat) -> ! {
    eprintln!(
        "Invalid date format. Use yyyy-mm-dd or dd/mm/yyyy. Got `{}`.",
        err.date_str
    );
    process::exit(1);
}

fn into_sgs_date_format(date_str: String) -> Result<String, InvalidDateStringFormat> {
    let str_bytes = date_str.as_bytes();

    if str_bytes.len() != 10 {
        return Err(InvalidDateStringFormat { date_str });
    }

    for b in str_bytes {
        if *b != b'-' && *b != b'/' && !b.is_ascii_digit() {
            return Err(InvalidDateStringFormat { date_str });
        }
    }

    if str_bytes[4] == b'-' && str_bytes[7] == b'-' {
        // yyyy-mm-dd
        let yyyy = &str_bytes[0..4];
        let mm = &str_bytes[5..7];
        let dd = &str_bytes[8..10];

        let result = vec![
            dd[0], dd[1], b'/', mm[0], mm[1], b'/', yyyy[0], yyyy[1], yyyy[2], yyyy[3],
        ];

        Ok(String::from_utf8(result).unwrap())
    } else if str_bytes[2] == b'/' && str_bytes[5] == b'/' {
        // dd/mm/yyyy
        Ok(date_str)
    } else {
        Err(InvalidDateStringFormat { date_str })
    }
}

#[test]
fn test_into_sgs_date_format() {
    assert!(into_sgs_date_format("2020-12-31".to_string()).unwrap() == "31/12/2020");
    assert!(into_sgs_date_format("31/12/2020".to_string()).unwrap() == "31/12/2020");
}

async fn execute_get_series(list: Vec<String>, from: String, to: String) {
    let from = match into_sgs_date_format(from) {
        Ok(date_string) => date_string,
        Err(err) => exit_with_error(err),
    };

    let to = match into_sgs_date_format(to) {
        Ok(date_string) => date_string,
        Err(err) => exit_with_error(err),
    };

    let mut vec_items: Vec<Item> = Vec::with_capacity(list.len());

    for id in list.iter() {
        match id.parse() {
            Ok(val) => {
                vec_items.push(Item::new(val));
            }
            Err(_) => {
                eprintln!(
                    "Invalid Serie ID: `{}`. Hint: all series must have a numeric ID.",
                    id
                );
                process::exit(1);
            }
        }
    }

    let item_list = sgslib::messages::ItemList { items: vec_items };

    let request = sgslib::messages::GetValoresSeriesXMLRequest {
        in0: item_list,
        in1: from,
        in2: to,
    };

    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);
    let response = sgs_client.get_valores_series_xml(request).await;

    let series_xml = response.unwrap().get_valores_series_xml_return.val;
    println!("{}", series_xml);
}

async fn execute_get_ultimo_valor(id: &str) {
    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);

    match id.parse() {
        Ok(id) => {
            let ultimo_valor = sgs_client
                .get_ultimo_valor_xml(sgslib::messages::GetUltimoValorXMLRequest {
                    in0: sgslib::messages::Item::new(id),
                })
                .await;
            let ultimo_valor = ultimo_valor.unwrap().get_ultimo_valor_xml_return;
            println!("{}", ultimo_valor);
        }
        Err(_) => {
            eprintln!(
                "Invalid Serie ID: `{}`. Hint: all series must have a numeric ID.",
                id
            );
            process::exit(1);
        }
    }
}
