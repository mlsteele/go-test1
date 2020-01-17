use failure::{Error, ResultExt, format_err};
use grep_searcher::{Searcher, Sink, SinkMatch};
use grep_regex::{RegexMatcher};
use ignore::Walk;
use std::io;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tee::TeeReader;

type Result<T> = std::result::Result<T, Error>;
type EResult = std::result::Result<(), Error>;

fn main() -> EResult {
    let matches = clap::App::new("go-test1")
    .version("1.0")
    .about("Go test runner")
    .arg(clap::Arg::with_name("name")
        .value_name("NAME")
        .help("Name of the go test. With or without the initial 'Test'.")
        .required(true))
    .arg(clap::Arg::with_name("log")
        .short("l")
        .value_name("LOG")
        .help("File in which to save the log. Defaults to /tmp/test.log"))
    .get_matches();

    macro_rules! arg_value {
        ( $name:expr ) => {
            matches.value_of($name).ok_or_else(|| format_err!("missing {}", $name.to_uppercase()))?
        };
    }

    let name = {
        let name = arg_value!("name");
        if name.starts_with("Test") {
            name.to_owned()
        } else {
            format!("Test{}", name)
        }
    };
    let log_path = matches.value_of("log").unwrap_or_else(|| "/tmp/test.log");
    println!("Test name: {}", name);
    println!("Command: go test -v -run \"^{}$\"", name);

    let mut path = find_file_for_test(&name)?.ok_or_else(|| format_err!("test not found: {}", name))?;
    println!("found test in file: {}", path.display());
    path.pop();

    let log_file = fs::File::create(log_path)?;
    let mut log_writer = io::BufWriter::new(log_file);
    let mut reader = duct::cmd!("go", "test", "-v", "-run", &format!("^{}$",name))
        .dir(path)
        .stderr_to_stdout()
        .reader()
        .with_context(|_|"attempted to exec 'go test'")?;
    io::copy(&mut TeeReader::new(&mut reader, &mut io::stdout()), &mut log_writer)?;
    log_writer.flush()?;

    Ok(())
}

fn find_file_for_test(name: &str) -> Result<Option<PathBuf>> {
    let mut searcher = Searcher::new();
    let matcher = RegexMatcher::new_line_matcher(&format!("func {}\\(.* \\*testing.T\\)", name))?;
    let mut sink = SimpleSink::new();
    for entry in Walk::new("./") {
        sink.reset();
        let entry = match entry {
            Ok(dent) => dent,
            Err(err) => {
                eprintln!("error walking files: {}", err);
                continue;
            }
        };
        if let Some(file_type) = entry.file_type() {
            if !file_type.is_file() {
                continue;
            }
        } else {
            continue;
        }
        if !entry.path().to_string_lossy().ends_with("_test.go") {
            continue;
        }
        let result = searcher.search_path(
            &matcher,
            entry.path(),
            &mut sink,
        );
        if let Err(err) = result {
            eprintln!("error searching file {}: {}", entry.path().display(), err);
        }
        if sink.found {
            return Ok(Some(entry.into_path()));
        }
    }
    Ok(None)
}

struct SimpleSink {found: bool}

impl SimpleSink {
    fn new() -> Self {Self {found: false}}

    pub fn reset(&mut self) {self.found = false}
}

impl Sink for SimpleSink {
    type Error = io::Error;

    fn matched(
        &mut self,
        _searcher: &Searcher,
        _mat: &SinkMatch
    ) -> std::result::Result<bool, Self::Error> {
        self.found = true;
        Ok(false) // stop the search
    }
}
