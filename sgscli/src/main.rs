use chrono::NaiveDate;
use clap::{Parser, Subcommand};
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

fn parse_sgs_date(date_str: &str) -> Result<NaiveDate, InvalidDateStringFormat> {
    let str_bytes = date_str.as_bytes();

    if str_bytes.len() != 10 {
        return Err(InvalidDateStringFormat {
            date_str: String::from(date_str),
        });
    }

    for b in str_bytes {
        if *b != b'-' && *b != b'/' && !b.is_ascii_digit() {
            return Err(InvalidDateStringFormat {
                date_str: String::from(date_str),
            });
        }
    }

    if str_bytes[4] == b'-' && str_bytes[7] == b'-' {
        // yyyy-mm-dd
        let yyyy = &date_str[0..4].parse().unwrap();
        let mm = &date_str[5..7].parse().unwrap();
        let dd = &date_str[8..10].parse().unwrap();

        Ok(NaiveDate::from_ymd(*yyyy, *mm, *dd))
    } else if str_bytes[2] == b'/' && str_bytes[5] == b'/' {
        // dd/mm/yyyy
        let dd = &date_str[0..2].parse().unwrap();
        let mm = &date_str[3..5].parse().unwrap();
        let yyyy = &date_str[6..10].parse().unwrap();

        Ok(NaiveDate::from_ymd(*yyyy, *mm, *dd))
    } else {
        Err(InvalidDateStringFormat {
            date_str: String::from(date_str),
        })
    }
}

#[test]
fn test_parse_sgs_date() {
    assert!(parse_sgs_date("2020-12-31").unwrap() == NaiveDate::from_ymd(2020, 12, 31));
    assert!(parse_sgs_date("31/12/2020").unwrap() == NaiveDate::from_ymd(2020, 12, 31));
}

async fn execute_get_series(list: Vec<String>, from: String, to: String) {
    let from = match parse_sgs_date(&from) {
        Ok(date_string) => date_string,
        Err(err) => exit_with_error(err),
    };

    let to = match parse_sgs_date(&to) {
        Ok(date_string) => date_string,
        Err(err) => exit_with_error(err),
    };

    let mut vec_series: Vec<i64> = Vec::with_capacity(list.len());

    for id in list.iter() {
        match id.parse() {
            Ok(val) => {
                vec_series.push(val);
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

    let sgs_client = sgslib::services::FachadaWSSGSService::new_client(Option::None);
    let response = sgslib::get_valores_series_xml(&sgs_client, &vec_series, from, to).await;

    match response {
        Ok(series_xml) => {
            println!("{}", series_xml);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            process::exit(1);
        }
    }
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

            match ultimo_valor {
                Ok(ultimo_valor) => {
                    let ultimo_valor = ultimo_valor.get_ultimo_valor_xml_return;
                    println!("{}", ultimo_valor);
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    process::exit(1);
                }
            }
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
