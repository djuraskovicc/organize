use std::error::Error;

pub struct CliParser {
    pub input: Option<String>,
    pub output: Option<String>,
    pub verbose_lvl: u8,
    pub help: bool,
}

impl CliParser {
    pub fn new() -> Self {
	Self {
            input: None,
            output: None,
            verbose_lvl: 0,
            help: false,
        }
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
		'h' => self.help = true,
		'v' => self.verbose_lvl += 1,
		'i' => self.input = Some(iter.next().ok_or("Missing value for --input")?),
		'o' => self.output = Some(iter.next().ok_or("Missing value for --output")?),
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
	    "help" => self.help = true,
	    "verbose" => self.verbose_lvl += 1,
	    "input" => self.input = Some(iter.next().ok_or("Missing value for --input")?),
	    "output" => self.output = Some(iter.next().ok_or("Missing value for --output")?),
	    _ => return Err(format!("Unknown flag --{}", name)),
	}

	Ok(())
    }
}
