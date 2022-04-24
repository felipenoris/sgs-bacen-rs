use chrono::NaiveDate;
use std::io::Read;
use std::str::FromStr;
use xml::reader::XmlEvent;
use yaserde::YaDeserialize;
use yaserde_derive::YaDeserialize;

#[derive(Debug, Clone)]
pub struct ItemDate {
    pub date: NaiveDate,
}

impl Default for ItemDate {
    fn default() -> Self {
        ItemDate {
            date: NaiveDate::from_ymd(1990, 1, 1),
        }
    }
}

impl YaDeserialize for ItemDate {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let res = reader.next_event()?;

        match res {
            XmlEvent::StartElement {
                name,
                attributes: _,
                namespace: _,
            } => {
                if name.local_name.as_str() != "DATA" {
                    return Err("Unexpected XML element.".to_string());
                }

                let res = reader.next_event()?;

                match res {
                    XmlEvent::Characters(str) => {
                        match NaiveDate::parse_from_str(&str, "%d/%m/%Y") {
                            Ok(date) => Ok(ItemDate { date }),
                            _ => Err("Unexpected date format in `DATE` element.".to_string()),
                        }
                    }
                    _ => {
                        println!("{:?}", res);
                        Err(String::from("Unexpected XML format."))
                    }
                }
            }

            _ => Err(String::from("Unexpected XML format")),
        }
    }
}

#[derive(Debug, Default, YaDeserialize, Clone)]
pub struct Item {
    #[yaserde(child, rename = "DATA")]
    pub data: ItemDate,

    #[yaserde(child, rename = "VALOR")]
    pub valor: Option<f64>,

    #[yaserde(child, rename = "BLOQUEADO")]
    pub bloqueado: bool,
}

#[derive(Debug, Default, YaDeserialize, Clone)]
pub struct Serie {
    #[yaserde(attribute, rename = "ID")]
    pub id: u32,

    #[yaserde(child, rename = "ITEM")]
    pub items: Vec<Item>,
}

#[derive(Debug, Default, YaDeserialize, Clone)]
#[yaserde(rename = "SERIES")]
pub struct Series {
    #[yaserde(child, rename = "SERIE")]
    pub series: Vec<Serie>,
}

impl FromStr for Series {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        yaserde::de::from_str(s)
    }
}

#[test]
fn parse_series() {
    let response_string = "<?xml version='1.0' encoding='ISO-8859-1'?>\n<SERIES>\n<SERIE ID='1'>\n\t\t<ITEM>\n\t\t\t<DATA>31/3/2022</DATA>\n\t\t\t<VALOR>4.7378</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>1/4/2022</DATA>\n\t\t\t<VALOR>4.6984</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>4/4/2022</DATA>\n\t\t\t<VALOR>4.6175</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>5/4/2022</DATA>\n\t\t\t<VALOR>4.6400</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>6/4/2022</DATA>\n\t\t\t<VALOR>4.6967</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>7/4/2022</DATA>\n\t\t\t<VALOR>4.7422</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>8/4/2022</DATA>\n\t\t\t<VALOR>4.7513</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>11/4/2022</DATA>\n\t\t\t<VALOR>4.7025</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>12/4/2022</DATA>\n\t\t\t<VALOR>4.6483</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>13/4/2022</DATA>\n\t\t\t<VALOR>4.6811</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>14/4/2022</DATA>\n\t\t\t<VALOR>4.7158</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>18/4/2022</DATA>\n\t\t\t<VALOR>4.6746</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>19/4/2022</DATA>\n\t\t\t<VALOR>4.6664</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>20/4/2022</DATA>\n\t\t\t<VALOR>4.6397</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t\t<ITEM>\n\t\t\t<DATA>22/4/2022</DATA>\n\t\t\t<VALOR>4.7326</VALOR>\n\t\t\t<BLOQUEADO>false</BLOQUEADO>\n\t\t</ITEM>\n\t</SERIE>\n</SERIES>";
    let series = Series::from_str(response_string).unwrap();
    println!("{:?}", series);
}
