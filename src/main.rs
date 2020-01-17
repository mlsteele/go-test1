extern crate clap;
use failure::{Error, ResultExt, format_err, bail};
use std::process::Command;
use std::os::unix::process::CommandExt;

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
    println!("Test name: {}", name);
    println!("Command: go test -v -run \"^{}$\"", name);
    Err(Command::new("go").args(&["test", "-v", "-run", &format!("^{}$",name)]).exec()).with_context(|_|"while running 'go test'")?;
    Ok(())
}
