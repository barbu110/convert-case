use clap::{App, AppSettings, Arg, crate_version};
use convert_case::{Case, Casing};

#[derive(Debug)]
enum Error {
    NoToCase,
    NoSuchCase,
    NoInput,
}

fn case_from_str(s: &str) -> Option<Case> {
    for case in Case::all_cases() {
        if format!("{:?}", case).to_case(Case::Flat) == s.to_string().to_case(Case::Flat) {
            return Some(case)
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let app = app();
    let matches = app.get_matches();

    let to_case_str = matches.value_of("to-case").ok_or(Error::NoToCase)?;
    let to_case = case_from_str(to_case_str).ok_or(Error::NoSuchCase)?;

    let input = matches.value_of("INPUT").ok_or(Error::NoInput)?;

    let converted = if let Some(from_case_str) = matches.value_of("from-case") {
        let from_case = case_from_str(from_case_str).ok_or(Error::NoSuchCase)?;
        input.from_case(from_case).to_case(to_case)
    } else {
        input.to_case(to_case)
    };

    println!("{}", converted);

    Ok(())
}

fn app<'a>() -> App<'a> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <davepurdum@pm.me>")
        .about("Converts strings to and from cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("INPUT")
                .help("String to convert.")
                .requires("to-case")
        )
        .arg(
            Arg::new("to-case")
                .short('t')
                .long("to")
                .value_name("CASE")
                .help("Case to convert string into.")
                .takes_value(true)
        )
        .arg(
            Arg::new("from-case")
                .short('f')
                .long("from")
                .value_name("CASE")
                .help("Case to convert string from.")
                .takes_value(true)
        )
}

#[cfg(test)]
mod test {
    use assert_cli::Assert;

    #[test]
    fn to_case() {
        Assert::main_binary()
            .with_args(&["-t", "snake", "myVarName"])
            .stdout()
            .is("my_var_name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["--to", "kebab", "myVarName"])
            .stdout()
            .is("my-var-name")
            .unwrap();
    }

    #[test]
    fn from_case() {
        Assert::main_binary()
            .with_args(&["-f", "camel", "-t", "snake", "myVar-Name"])
            .stdout()
            .is("my_var-name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["-t", "camel", "--from", "kebab", "my-Var-Name_longer"])
            .stdout()
            .is("myVarName_longer")
            .unwrap();
    }

    #[test]
    fn to_case_not_lower() {
        Assert::main_binary()
            .with_args(&["--to", "KeBAB", "myVarName"])
            .stdout()
            .is("my-var-name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["--from", "KeBAB", "-t", "snake", "my-bad-VARiable"])
            .stdout()
            .is("my_bad_variable")
            .unwrap();
    }

}
