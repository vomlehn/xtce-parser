
//use xml::Reader;
use std::io::{self, BufRead, BufReader, Read};
use xml::reader::{XmlEvent};
pub use xml::reader::{EventReader};

pub struct LineReader<R: Read> {
    inner: BufReader<R>,
    line: usize,
}

impl<R: Read> LineReader<R> {
    pub fn new(inner: R) -> Self {
        LineReader {
            inner: BufReader::new(inner),
            line: 1,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl<R: Read> Read for LineReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.line += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
println!("self.line {}", self.line);
        Ok(bytes_read)
    }
}

/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"<root>\n<child>Value</child>\n</root>"#;
    let reader = LineReader::new(xml.as_bytes());

    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.trim_text(true);

    for event in xml_reader.events() {
        match event {
            Ok(xml::events::Event::Start(ref e)) => {
                let line = xml_reader.reader().line();
                println!("Start element {:?} at line {}", e.name(), line);
            }
            _ => {}
        }
    }

    Ok(())
}
*/
