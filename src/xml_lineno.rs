
//use xml::Reader;
use crate::basic::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, BufReader, Read};

pub struct LineReader<R: Read> {
    inner: BufReader<R>,
    line: Rc<RefCell<usize>>,
}

impl<R: Read> LineReader<R> {
    pub fn new(inner: R) -> Self {
        LineReader {
            inner: BufReader::new(inner),
            line: Rc::new(RefCell::new(1)),
        }
    }

    pub fn line_ref(&self) -> Rc<RefCell<usize>> {
        Rc::clone(&self.line)
    }
}

impl<R: Read> Read for LineReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        let mut line = self.line.borrow_mut();
        *line += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
        Ok(bytes_read)
    }
}
