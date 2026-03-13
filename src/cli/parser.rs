use std::error::Error;
use crate::cli::args::Flag;

pub struct CliParser {
    pub flags: Vec<Flag>,
}

impl CliParser {
    pub fn new() -> Self {
	Self { flags: Vec::with_capacity(5) }
    }

    pub fn read_args<I>(&mut self, args: I) -> Result<(), Box<dyn Error>> 
    where
	I: IntoIterator<Item = String>,
    {
	let mut iter = args.into_iter();
	
	while let Some(arg) = iter.next() {
	    if arg.starts_with("--") {
		self.parse_long(&arg, &mut iter)?;
	    } else if arg.starts_with('-') {
		self.parse_short(&arg, &mut iter)?;
	    }
	}

	Ok(())
    }

    fn parse_short<I>(&mut self, arg: &str, iter: &mut I) -> Result<(), String> 
    where
	I: Iterator<Item = String>
    {
	for c in arg.trim_start_matches('-').chars() {
	    match c {
		'h' => self.flags.push(Flag::Help),
		'v' => self.flags.push(Flag::Verbose { level: 1 }),
		'o' => {
		    let value = iter.next().ok_or("Missing value for -o")?;
		    self.flags.push(Flag::Output { 
			path: value.clone(),
		    });
		},
		_ => return Err(format!("Unknown flag: -{}", c)),
	    }
	}
	
	Ok(())
    }

    fn parse_long<I>(&mut self, arg: &str, iter: &mut I) -> Result<(), String> 
    where
	I: Iterator<Item = String>,
    {
	let name = arg.trim_start_matches("--");

	match name {
	    "help" => self.flags.push(Flag::Help),
	    "verbose" => self.flags.push(Flag::Verbose { level: 1 }),
	    "output" => {
		let value = iter.next().ok_or("Mission value for --output")?;
		self.flags.push(Flag::Output {
		    path: value.clone(),
		});
	    },
	    _ => return Err(format!("Unknown flag --{}", name)),
	}

	Ok(())
    }
}
