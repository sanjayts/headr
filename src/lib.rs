use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args(cmd_args: Vec<String>) -> MyResult<Config> {
    let matches = App::new("headr")
        .author("sanjayts")
        .version("1.0.0")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .default_value("-")
                .multiple_values(true),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .multiple_values(false)
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .value_name("BYTES")
                .long("bytes")
                .takes_value(true)
                .multiple_values(false)
                .conflicts_with("lines"),
        )
        .get_matches_from(cmd_args);

    let lines = get_field("lines", "illegal line count", &matches)?;
    let bytes = get_field("bytes", "illegal byte count", &matches)?;

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();

    let config = Config {
        files,
        lines: lines.unwrap(),
        bytes,
    };
    Ok(config)
}

pub fn run(config: &Config) -> MyResult<()> {
    let has_multiple_files = config.files.len() > 1;
    for (num, file_name) in config.files.iter().enumerate() {
        match open_file(file_name) {
            Ok(buf) => {
                // Print a \n just before printing out the file header IFF it's not the first one.
                // Ensure the file headers are only printed when we are dealing with multiple files.
                if has_multiple_files {
                    println!("{}==> {} <==", if num > 0 { "\n" } else { "" }, file_name);
                }
                match config.bytes {
                    Some(cnt) => handle_bytes(buf, &cnt)?,
                    None => handle_lines(buf, &config.lines)?,
                }
            }
            Err(e) => eprintln!("headr: {}: {}", file_name, e.to_string()),
        }
    }
    Ok(())
}

fn handle_bytes(reader: Box<dyn BufRead>, count: &usize) -> MyResult<()> {
    // Ensure we are not reading more than necessary by upper bounding the reader 'read' limit
    let count = *count;
    let mut reader = reader.take(count as u64);

    let mut byte_buf = vec![0; count];
    let bytes_read = reader.read(byte_buf.as_mut_slice())?;

    // This is to handle cases wherein the bytes requested is more than the total
    // number of bytes in the file
    byte_buf.truncate(bytes_read);

    // Don't use println; we don't need an additional new line after printing the bytes
    print!("{}", String::from_utf8_lossy(byte_buf.as_mut_slice()));
    Ok(())
}

fn handle_lines(mut reader: Box<dyn BufRead>, count: &usize) -> MyResult<()> {
    // It's very important to ensure that we don't use .lines() iterator but explicitly invoke
    // the .read_line() method. This is because the iterator implementation strips off all CR/LF
    // characters which is something we don't want. As an example, using .lines iterator breaks
    // when reading a file which has "abcd\r\n". Using the line yielded by the iterator inside our
    // println() will output 'abcd\n" on our stdout whereas we want "abcd\r\n" on stdout.
    let mut buf = String::new();
    for _ in 0..*count {
        let bytes_read = reader.read_line(&mut buf)?;
        if bytes_read == 0 {
            // Ensure that we don't waste time looping 'count' times when we don't have anything
            break;
        }
        print!("{}", buf);
        buf.clear();
    }
    Ok(())
}

fn open_file(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn get_field(arg_id: &str, err_msg: &str, matches: &ArgMatches) -> MyResult<Option<usize>> {
    matches
        .get_one::<String>(arg_id)
        .map(|s| parse_positive(s))
        .transpose()
        .map_err(|e| format!("{} -- {}", err_msg, e).into())
}

fn parse_positive(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[cfg(test)]
mod lib_tests {
    use crate::parse_positive;
    use std::assert_eq;

    #[test]
    fn test_parse_positive() {
        let p42 = parse_positive("42");
        assert!(p42.is_ok());
        assert_eq!(p42.unwrap(), 42);

        let pstr = parse_positive("foo");
        assert!(pstr.is_err());
        assert_eq!(pstr.unwrap_err().to_string(), "foo");

        let p0 = parse_positive("0");
        assert!(p0.is_err());
        assert_eq!(p0.unwrap_err().to_string(), "0");
    }
}
