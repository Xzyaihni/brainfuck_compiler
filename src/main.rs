use std::process;
use std::env;
use std::fs;

mod compiler;


fn help_message() -> !
{
    let executable = env::args().next().expect("all programs have a name");
    eprintln!("usage: {executable} [args] filepath");
    process::exit(1);
}

struct Config
{
    filepath: String,
    program_name: String
}

impl Config
{
    fn parse(args: impl Iterator<Item=String>) -> Result<Self, String>
    {
        let mut filepath = String::new();
        let mut program_name = "out".to_string();

        let mut args = args.skip(1).peekable();
        while let Some(arg) = args.next()
        {
            if args.peek().is_none()
            {
                filepath = arg;
                break;
            }

            match arg.as_str()
            {
                "-o" | "--output" =>
                {
                    program_name = args.next().ok_or(format!("{arg} has no argument"))?;
                },
                _ => return Err(format!("invalid argument: {arg}"))
            }
        }

        if filepath.is_empty()
        {
            return Err("empty filepath".to_string());
        }

        Ok(Config{filepath, program_name})
    }
}

fn main()
{

    let config = Config::parse(env::args()).unwrap_or_else(|err|
    {
        eprintln!("{err}");
        help_message();
    });

    compiler::compile(&fs::read(config.filepath).unwrap_or_else(|err|
    {
        eprintln!("cant read file: {err}");
        help_message();
    }), config.program_name).unwrap_or_else(|err|
    {
        eprintln!("compiler error: {err}");
    });
}