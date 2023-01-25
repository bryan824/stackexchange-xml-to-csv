use std::{
    collections::HashSet,
    error::Error,
};

use quick_xml::{
    events::Event,
    reader::Reader,
};

fn main() -> Result<(), Box<dyn Error>> {
    let source = std::env::args().nth(1).expect("no source path given");
    let target = std::env::args().nth(2).expect("no target path given");
    let mut reader = Reader::from_file(&source).unwrap();

    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut column_set = HashSet::new();
    // Initial loop is to get the schema of the csv.
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"row" => {
                        e.attributes().for_each(|a| {
                            let key = a.unwrap().key.0;
                            column_set.insert(String::from_utf8(key.to_owned()).unwrap());
                        });
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }
    // Second loop is to insert data.
    let mut wtr = csv::Writer::from_path(target)?;
    let columns: Vec<String> = column_set.into_iter().collect();
    wtr.write_record(&columns)?;
    let mut reader = Reader::from_file(&source).unwrap();
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"row" => {
                        let empty_str = "".as_bytes().into();
                        let mut record = vec![empty_str; columns.len()];

                        e.attributes().for_each(|a| {
                            let pos = columns
                                .iter()
                                .position(|r| r.as_bytes() == a.as_ref().unwrap().key.0)
                                .unwrap();
                            record[pos] = a.as_ref().unwrap().value.clone()
                        });

                        wtr.write_record(record)?;
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }

    wtr.flush()?;
    Ok(())
}
