extern crate reqwest;
extern crate select;
extern crate encoding;

// importation syntax
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize)]
struct Record{
    index: i32,
    nombre : String,
    poblacion : String,
    residencia :String,
    profesion : String,
    expediente : String,
    archivo: String,
    fondo: String,
    serie: String,
    signatura: String,
    fecha: String,
    paginas: String,
    tipologia: String,
    observaciones: String,
}

fn print_record( record : Record) -> Result<()>{
    let j = serde_json::to_string(&record)?;
    println!("{}", j);
    Ok(())
}

fn main() {

    let index : i32 = std::env::args()
        .nth(1).expect("Neceisto el inicio").parse().unwrap();

    let end : i32 = std::env::args()
        .nth(2).expect("Necesito el final").parse().unwrap();

    for n in index..end{
        let url = format!("http://pares.mcu.es/victimasGCFPortal/detalle.form?idpersona={}",n);

        let resp = reqwest::blocking::get(&url).unwrap();

        if resp.status().is_success() {
            let body = resp.text_with_charset("ISO-8859-1").unwrap();

            let document = Html::parse_document(&body);

            let detalle_selector = Selector::parse("table[summary='Detalle']").unwrap();
            let expediente_selector = Selector::parse("table[summary='Expediente']").unwrap();

            let tr_selector = Selector::parse("tr").unwrap();
            let td_selector = Selector::parse("td").unwrap();
            let strong_selector = Selector::parse("strong").unwrap();

            let mut record = Record{
                index: n,
                nombre: "".to_owned(),
                poblacion: "".to_owned(),
                residencia: "".to_owned(),
                profesion: "".to_owned(),
                expediente : "".to_owned(),
                archivo: "".to_owned(),
                fondo: "".to_owned(),
                serie: "".to_owned(),
                signatura: "".to_owned(),
                fecha: "".to_owned(),
                paginas: "".to_owned(),
                tipologia: "".to_owned(),
                observaciones: "".to_owned(),
            };

            if document.select(&detalle_selector).count() > 0{
                let total_tr = document.select(&detalle_selector)
                    .next().unwrap().select(&tr_selector).count();

                let detalle = document.select(&detalle_selector).next().unwrap();

                let mut trs = detalle.select(&tr_selector);

                let nombre = trs.nth(0).unwrap()
                            .select(&td_selector).nth(0).unwrap()
                            .select(&strong_selector).nth(0)
                            .unwrap();
                record.nombre = String::from(nombre.inner_html().trim());

                let _otros = trs.nth(0).unwrap();

                let poblacion = trs.nth(0).unwrap()
                    .select(&td_selector).nth(0).unwrap().text().next().unwrap_or("");
                record.poblacion = String::from(poblacion.trim());

                if total_tr == 5 {
                    let residencia = trs.nth(0).unwrap()
                        .select(&td_selector).nth(0).unwrap().text().next().unwrap_or("");
                    record.residencia = String::from(residencia.trim());
                }

                let profesion = trs.nth(0).unwrap()
                    .select(&td_selector).nth(0).unwrap().text().next().unwrap_or("");
                record.profesion = String::from(profesion.trim());

            }

            if document.select(&expediente_selector).count() > 0 {
                let th_selector = Selector::parse("th").unwrap();

                let expediente = document.select(&expediente_selector).next().unwrap();
                let trs = expediente.select(&tr_selector);
                for tr in trs{
                    let title = tr.select(&th_selector).next().unwrap().inner_html();
                    let value = tr.select(&td_selector).next().unwrap().inner_html();
                    match title.trim(){
                        "Archivo" => record.archivo = String::from(value.trim()),
                        "Fondo" => record.fondo = String::from(value.trim()),
                        "Serie" => record.serie = String::from(value.trim()),
                        "Signatura" => record.signatura = String::from(value.trim()),
                        "Fecha de expediente" => record.fecha = String::from(value.trim()),
                        "Tipología" => record.tipologia = String::from(value.trim()),
                        "Observaciones" => record.observaciones = String::from(value.trim()),
                        _  => record.paginas = String::from(value.trim()),
                    }
                }
            }

            print_record(record);
        }
    }

}
