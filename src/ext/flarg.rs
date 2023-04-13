//! A minimalist library for parsing command line arguments.
// mod from https://github.com/dmulholl/arguably

use std::collections::HashMap;
use std::error;
use std::fmt;

// We create a single Opt instance for each registered option, i.e. each call to `.option()`.
#[derive(Debug)]
struct Opt {
	values: Vec<String>,
	default: String,
	desc: (String, Option<String>),
}

// We create a single Flag instance for each registered flag, i.e. each call to `.flag()`.
#[derive(Debug)]
struct Flag {
	count: usize,
	desc: (String, Option<String>),
}

/// An ArgParser instance can be intialized using the builder pattern.
#[derive(Debug, Default)]
pub struct ArgParser {
	help_head: Option<String>,

	options: Vec<Opt>,
	option_map: HashMap<String, usize>,

	flags: Vec<Flag>,
	flag_map: HashMap<String, usize>,

	/// Stores application path.
	pub app_path: Option<String>,

	/// Stores positional arguments.
	pub args: Vec<String>,
}

fn dash(s: &str) -> String {
	let mut out = Vec::new();
	for alias in s.split_whitespace() {
		out.push(match alias.chars().count() {
			1 => format!("-{}", alias),
			_ => format!("--{}", alias),
		});
	}
	out.join(", ")
}

impl ArgParser {
	/// Creates a new ArgParser instance.
	pub fn new() -> ArgParser {
		ArgParser {
			app_path: std::env::args().next(),
			..Default::default()
		}
	}

	pub fn print_help(&self) {
		println!("{}", self.help_head.as_ref().unwrap().trim());
		self.flags.iter().for_each(|f| {
			match f.desc.1.as_ref() {
				Some(str) => println!("  {}\n         {}", dash(f.desc.0.as_str()), str),
				_ => println!("  {}\n", dash(f.desc.0.as_str())),
			};
		});
		self.options.iter().for_each(|f| {
			match f.desc.1.as_ref() {
				Some(str) => println!(
					"  {0}={1}\n         {2} (default: {1})",
					dash(f.desc.0.as_str()),
					f.default,
					str
				),
				_ => println!("  {0}={1}\n         default: {1}", dash(f.desc.0.as_str()), f.default),
			};
		});
		std::process::exit(0);
	}

	/// Sets the parser's helptext string.
	pub fn helptext<S>(mut self, text: S) -> Self
	where
		S: Into<String>,
	{
		self.help_head = Some(text.into());
		self
	}

	/// Registers a new option with description.
	pub fn option_with(mut self, name: &str, default: &str, description: &str) -> Self {
		self.options.push(Opt {
			values: Vec::new(),
			default: String::from(default),
			desc: (name.into(), Some(description.into())),
		});
		let index = self.options.len() - 1;
		for alias in name.split_whitespace() {
			self.option_map.insert(alias.to_string(), index);
		}
		self
	}

	/// Registers a new flag with description.
	pub fn flag_with(mut self, name: &str, description: &str) -> Self {
		self.flags.push(Flag {
			count: 0,
			desc: (name.into(), Some(description.into())),
		});
		let index = self.flags.len() - 1;
		for alias in name.split_whitespace() {
			self.flag_map.insert(alias.to_string(), index);
		}
		self
	}

	/// Returns the value of the named option.
	pub fn value(&self, name: &str) -> String {
		if let Some(index) = self.option_map.get(name) {
			if let Some(value) = self.options[*index].values.last() {
				return value.to_string();
			}
			return self.options[*index].default.clone();
		}
		panic!("'{}' is not a registered option name", name);
	}

	/// Returns the number of times the named flag or option was found.
	pub fn count(&self, name: &str) -> usize {
		if let Some(index) = self.flag_map.get(name) {
			return self.flags[*index].count;
		}
		if let Some(index) = self.option_map.get(name) {
			return self.options[*index].values.len();
		}
		panic!("'{}' is not a registered flag or option name", name);
	}

	/// Returns `true` if the named flag or option was found.
	pub fn found(&self, name: &str) -> bool {
		self.count(name) > 0
	}

	/// Parse the program's command line arguments.
	pub fn parse(&mut self) -> Result<(), Error> {
		let mut strings = Vec::<String>::new();
		for os_string in std::env::args_os().skip(1) {
			if let Ok(string) = os_string.into_string() {
				strings.push(string);
			} else {
				return Err(Error::InvalidUnicode);
			}
		}
		let mut stream = ArgStream::new(strings);
		self.parse_argstream(&mut stream)?;
		Ok(())
	}

	fn parse_argstream(&mut self, argstream: &mut ArgStream) -> Result<(), Error> {
		while argstream.has_next() {
			match argstream.next().as_str() {
				"--" => {
					while argstream.has_next() {
						self.args.push(argstream.next());
					}
				}
				arg if arg.starts_with("--") => match arg.contains("=") {
					true => self.handle_equals_opt(arg)?,
					false => self.handle_long_opt(arg, argstream)?,
				},
				"-" => self.args.push("-".to_string()),
				arg if arg.starts_with("-") => match arg.contains("=") {
					true => self.handle_equals_opt(arg)?,
					false => self.handle_short_opt(arg, argstream)?,
				},
				arg => self.args.push(arg.to_string()),
			}
		}
		Ok(())
	}

	fn handle_long_opt(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
		if let Some(index) = self.flag_map.get(&arg[2..]) {
			self.flags[*index].count += 1;
			return Ok(());
		}

		if let Some(index) = self.option_map.get(&arg[2..]) {
			if !argstream.has_next() {
				return Err(Error::MissingValue(format!("missing value for {}", arg)));
			}

			self.options[*index].values.push(argstream.next());
			return Ok(());
		}

		return Err(Error::InvalidName(format!(
			"{arg} is not a recognised flag or option name",
		)));
	}

	fn handle_short_opt(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
		for c in arg.chars().skip(1) {
			if let Some(index) = self.flag_map.get(&c.to_string()) {
				self.flags[*index].count += 1;
				continue;
			}

			if let Some(index) = self.option_map.get(&c.to_string()) {
				if !argstream.has_next() {
					return Err(Error::MissingValue(match arg.chars().count() > 2 {
						true => format!("missing value for '{}' in {}", c, arg),
						false => format!("missing value for {}", arg),
					}));
				}

				self.options[*index].values.push(argstream.next());
				continue;
			}

			return Err(Error::InvalidName(match arg.chars().count() > 2 {
				true => format!("'{}' in {} is not a recognised flag or option name", c, arg),
				false => format!("{} is not a recognised flag or option name", arg),
			}));
		}
		Ok(())
	}

	fn handle_equals_opt(&mut self, arg: &str) -> Result<(), Error> {
		let splits: Vec<&str> = arg.splitn(2, '=').collect();
		let name = splits[0];
		let value = splits[1];

		if let Some(index) = self.option_map.get(name.trim_start_matches('-')) {
			if value == "" {
				return Err(Error::MissingValue(format!("missing value for {name}")));
			}

			self.options[*index].values.push(value.to_string());
			return Ok(());
		}

		return Err(Error::InvalidName(format!("{name} is not a recognised option name")));
	}
}

//

// This type functions as a wrapper to make the input argument vector available as a stream.
struct ArgStream {
	args: Vec<String>,
	index: usize,
}

impl ArgStream {
	fn new(args: Vec<String>) -> ArgStream {
		ArgStream { args, index: 0 }
	}

	fn has_next(&self) -> bool {
		self.index < self.args.len()
	}

	fn next(&mut self) -> String {
		self.index += 1;
		self.args[self.index - 1].clone()
	}
}

//

/// Error types returned by the library.
#[derive(Debug)]
pub enum Error {
	/// Returned when the parser detects an unregistered flag, option, or command name.
	InvalidName(String),

	/// Returned when the parser detects an option with a missing value.
	MissingValue(String),

	/// Returned when the command line arguments are not valid unicode strings.
	InvalidUnicode,
}

impl error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::InvalidName(msg) => write!(f, "Error: {}", msg),
			Error::MissingValue(msg) => write!(f, "Error: {}", msg),
			Error::InvalidUnicode => write!(f, "Error: arguments are not valid unicode strings"),
		}
	}
}

impl Error {
	/// Prints an error message to `stderr` and exits with a non-zero status code.
	pub fn exit(self) -> ! {
		eprintln!("{}.", self);
		std::process::exit(1);
	}
}
