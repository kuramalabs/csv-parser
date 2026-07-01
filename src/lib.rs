#![allow(unused, dead_code)]
pub struct CsvReader<'a> {
    input: &'a str,
}

impl<'a> CsvReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

#[derive(Debug)]
pub struct CsvRow<'a> {
    line: Option<&'a str>,
    delimiter: char,
}

impl<'a> Iterator for CsvReader<'a> {
    type Item = CsvRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // agar \n hua to split and split based on the comma ..
        // agar direct line hua to seedha split based on comma

        if self.input.is_empty() {
            return None;
        }

        let pos = self.input.find("\n").unwrap_or(self.input.len());

        let line = &self.input[..pos];

        self.input = if pos < self.input.len() { &self.input[pos + 1..] } else { "" };

        Some(CsvRow {
            line: Some(line),
            delimiter: ',',
        })
    }
}

impl<'a> Iterator for CsvRow<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.line?;

        match line.find(self.delimiter) {
            Some(pos) => {
                let field = &line[..pos];
                self.line = Some(&line[pos + self.delimiter.len_utf8()..]);
                Some(field)
            }
            None => {
                self.line = None;
                Some(line)
            }
        }
    }
}

#[test]
fn basic() {
    let input = "a,b,c\n1,2,3";
    let mut reader = CsvReader::new(input);

    let row1: Vec<&str> = reader.next().unwrap().collect();
    assert_eq!(row1, vec!["a", "b", "c"]);

    let row2: Vec<&str> = reader.next().unwrap().collect();
    assert_eq!(row2, vec!["1", "2", "3"]);

    assert!(reader.next().is_none());
}

#[test]
fn iterator_outlives_reader() {
    let input = String::from("a,b,c");
    let fields: Vec<&str>;
    {
        let mut reader = CsvReader::new(&input);
        let row = reader.next().unwrap();
        fields = row.collect();
    }
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn empty_input() {
    let mut reader = CsvReader::new("");
    assert!(reader.next().is_none());
}

#[test]
fn single_field() {
    let mut reader = CsvReader::new("hello");
    let row: Vec<&str> = reader.next().unwrap().collect();
    assert_eq!(row, vec!["hello"]);
}

#[test]
fn empty_fields() {
    let input = "a,,c";
    let row: Vec<&str> = CsvReader::new(input).next().unwrap().collect();
    assert_eq!(row, vec!["a", "", "c"]);
}

#[test]
fn can_use_row_after_advancing_reader() {
    let input = "a,b,c\n1,2,3";
    let mut reader = CsvReader::new(input);

    let row1 = reader.next().unwrap();
    let row2 = reader.next().unwrap();

    let fields1: Vec<&str> = row1.collect();
    let fields2: Vec<&str> = row2.collect();

    assert_eq!(fields1, vec!["a", "b", "c"]);
    assert_eq!(fields2, vec!["1", "2", "3"]);
}

#[test]
fn trailing_newline() {
    let input = "a,b,c\n";
    let mut reader = CsvReader::new(input);
    let row: Vec<&str> = reader.next().unwrap().collect();
    assert_eq!(row, vec!["a", "b", "c"]);
    assert!(reader.next().is_none());
}

#[test]
fn trailing_comma() {
    let input = "a,b,";
    let row: Vec<&str> = CsvReader::new(input).next().unwrap().collect();
    assert_eq!(row, vec!["a", "b", ""]);
}

#[test]
fn single_comma() {
    let row: Vec<&str> = CsvReader::new(",").next().unwrap().collect();
    assert_eq!(row, vec!["", ""]);
}
