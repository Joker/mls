#![allow(dead_code)]
//! A minimalist library for parsing command line arguments.
//!
//! https://github.com/dmulholl/arguably
//!
//! ## Features
//!
//! * Long-form boolean flags with single-character shortcuts: `--flag`, `-f`.
//! * Long-form string-valued options with single-character shortcuts: `--option <arg>`, `-o <arg>`.
//! * Condensed short-form options: `-abc <arg> <arg>`.
//! * Automatic `--help` and `--version` flags.
//! * Support for multivalued options.
//! * Support for git-style command interfaces with arbitrarily-nested commands.
//!
//! ## Example
//!
//! ```
//! # use flarg::ArgParser;
//! let mut parser = ArgParser::new()
//!     .helptext("Usage: foobar...")
//!     .version("1.0")
//!     .option("bar b", "default")
//!     .flag("foo f");
//!
//! if let Err(err) = parser.parse() {
//!     err.exit();
//! }
//!
//! if parser.found("foo") {
//!     println!("Flag --foo/-f found.");
//! }
//!
//! println!("Option --bar/-b has value: {}", parser.value("bar"));
//!
//! for arg in parser.args {
//!     println!("Arg: {}", arg);
//! }
//! ```

use std::collections::HashMap;
use std::error;
use std::fmt;

/// Error types returned by the library.
#[derive(Debug)]
pub enum Error {
	/// Returned when the parser detects an unregistered flag, option, or command name.
	InvalidName(String),

	/// Returned when the parser detects an option with a missing value.
	MissingValue(String),

	/// Returned when the parser detects a help command with a missing argument.
	MissingHelpArg,

	/// Returned when the command line arguments are not valid unicode strings.
	InvalidUnicode,
}

impl error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::InvalidName(msg) => write!(f, "Error: {}", msg),
			Error::MissingValue(msg) => write!(f, "Error: {}", msg),
			Error::MissingHelpArg => write!(f, "Error: missing argument for the help command"),
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

/// An ArgParser instance can be intialized using the builder pattern.
///
/// ```
/// # use flarg::ArgParser;
/// let mut parser = ArgParser::new()
///     .helptext("Usage: appname...")
///     .version("1.0")
///     .option("bar b", "default")
///     .flag("foo f");
/// ```
#[derive(Debug)]
pub struct ArgParser {
	helptext: Option<String>,
	version: Option<String>,
	options: Vec<Opt>,
	option_map: HashMap<String, usize>,
	flags: Vec<Flag>,
	flag_map: HashMap<String, usize>,
	commands: Vec<ArgParser>,
	command_map: HashMap<String, usize>,
	// callback: Option<fn(&str, &ArgParser)>,

	pub app_name: Option<String>,

	/// Stores positional arguments.
	pub args: Vec<String>,

	/// Stores the command name, if a command was found.
	pub cmd_name: Option<String>,

	/// Stores the command's `ArgParser` instance, if a command was found.
	pub cmd_parser: Option<Box<ArgParser>>,

	/// Deprecated. Use `.enable_help_command()` instead.
	pub cmd_help: bool,
}

impl ArgParser {
	/// Creates a new ArgParser instance.
	pub fn new() -> ArgParser {
		ArgParser {
			helptext: None,
			version: None,
			app_name: std::env::args().next(),
			args: Vec::new(),
			options: Vec::new(),
			option_map: HashMap::new(),
			flags: Vec::new(),
			flag_map: HashMap::new(),
			commands: Vec::new(),
			command_map: HashMap::new(),
			// callback: None,
			cmd_name: None,
			cmd_parser: None,
			cmd_help: false,
		}
	}

	/// Sets the parser's helptext string. Supplying a helptext string activates support
	/// for an automatic `--help` flag, also a `-h` shortcut if not registered by another
	/// option.
	///
	/// ```
	/// # use flarg::ArgParser;
	/// let mut parser = ArgParser::new()
	///     .helptext("Usage: appname...");
	/// ```
	pub fn helptext<S>(mut self, text: S) -> Self
	where
		S: Into<String>,
	{
		self.helptext = Some(text.into());
		self
	}

	/// Sets the parser's version string. Supplying a version string activates support
	/// for an automatic `--version` flag, also a `-v` shortcut if not registered by another
	/// option.
	///
	/// ```
	/// # use flarg::ArgParser;
	/// let mut parser = ArgParser::new()
	///     .version("1.0");
	/// ```
	pub fn version<S>(mut self, text: S) -> Self
	where
		S: Into<String>,
	{
		self.version = Some(text.into());
		self
	}

	/// Registers a new option. The `name` parameter accepts an unlimited number of
	/// space-separated aliases and single-character shortcuts. The `default` value
	/// will be used if the option is not found.
	///
	/// ```
	/// # use flarg::ArgParser;
	/// let mut parser = ArgParser::new()
	///     .option("foo f", "default value");
	/// ```
	pub fn option(mut self, name: &str, default: &str) -> Self {
		self.options.push(Opt {
			values: Vec::new(),
			default: String::from(default),
		});
		let index = self.options.len() - 1;
		for alias in name.split_whitespace() {
			self.option_map.insert(alias.to_string(), index);
		}
		self
	}

	/// Registers a new flag. The `name` parameter accepts an unlimited number of
	/// space-separated aliases and single-character shortcuts.
	///
	/// ```
	/// # use flarg::ArgParser;
	/// let mut parser = ArgParser::new()
	///     .flag("foo f");
	/// ```
	pub fn flag(mut self, name: &str) -> Self {
		self.flags.push(Flag { count: 0 });
		let index = self.flags.len() - 1;
		for alias in name.split_whitespace() {
			self.flag_map.insert(alias.to_string(), index);
		}
		self
	}

	/// Registers a new command. The `name` parameter accepts an unlimited number of
	/// space-separated aliases. The command's helptext, flags, and options can be
	/// registered on the command's ArgParser instance.
	///
	/// ```
	/// # use flarg::ArgParser;
	/// let mut parser = ArgParser::new()
	///     .helptext("Usage: appname...")
	///     .command("cmdname", ArgParser::new()
	///         .helptext("Usage: appname cmdname...")
	///         .flag("cmdflag")
	///     );
	/// ```
	pub fn command(mut self, name: &str, cmd_parser: ArgParser) -> Self {
		if cmd_parser.helptext.is_some() {
			self.cmd_help = true;
		}
		self.commands.push(cmd_parser);
		let index = self.commands.len() - 1;
		for alias in name.split_whitespace() {
			self.command_map.insert(alias.to_string(), index);
		}
		self
	}

	/// This boolean switch toggles support for an automatic `help` command that prints subcommand
	/// helptext. The value defaults to `false` but gets toggled automatically to `true` whenever a
	/// command with helptext is registered. You can use this method to disable the feature if
	/// required.
	pub fn enable_help_command(mut self, enable: bool) -> Self {
		self.cmd_help = enable;
		self
	}

	/// Registers a callback function on a command parser. If the command is found the
	/// function will be called and passed the command name and a reference to the
	/// command's `ArgParser` instance.
	// pub fn callback(mut self, f: fn(&str, &ArgParser)) -> Self {
	// 	self.callback = Some(f);
	// 	self
	// }

	/// Returns the value of the named option. Returns the default value registered
	/// with the option if the option was not found. Any of the option's registered
	/// aliases or shortcuts can be used for the `name` parameter.
	/// (This function will panic if `name` is not a registered option name.)
	pub fn value(&self, name: &str) -> String {
		if let Some(index) = self.option_map.get(name) {
			if let Some(value) = self.options[*index].values.last() {
				return value.to_string();
			}
			return self.options[*index].default.clone();
		}
		panic!("'{}' is not a registered option name", name);
	}

	/// Returns the named option's list of values. Any of the option's registered
	/// aliases or shortcuts can be used for the `name` parameter.
	/// (This function will panic if `name` is not a registered option name.)
	pub fn values(&self, name: &str) -> Vec<String> {
		if let Some(index) = self.option_map.get(name) {
			return self.options[*index].values.clone();
		}
		panic!("'{}' is not a registered option name", name);
	}

	/// Returns the number of times the named flag or option was found. Any registered
	/// alias or shortcut can be used for the `name` parameter.
	/// (This function will panic if `name` is not a registered flag or option name.)
	pub fn count(&self, name: &str) -> usize {
		if let Some(index) = self.flag_map.get(name) {
			return self.flags[*index].count;
		}
		if let Some(index) = self.option_map.get(name) {
			return self.options[*index].values.len();
		}
		panic!("'{}' is not a registered flag or option name", name);
	}

	/// Returns `true` if the named flag or option was found. Any registered alias or
	/// shortcut can be used for the `name` parameter.
	/// (This function will panic if `name` is not a registered flag or option name.)
	pub fn found(&self, name: &str) -> bool {
		self.count(name) > 0
	}

	/// Parse the program's command line arguments.
	///
	/// ```
	/// # let mut parser = flarg::ArgParser::new();
	/// if let Err(err) = parser.parse() {
	///     err.exit();
	/// }
	/// ```
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

	/// Parse a vector of arguments.
	pub fn parse_vec(&mut self, args: Vec<&str>) -> Result<(), Error> {
		let strings = args.iter().map(|s| s.to_string()).collect();
		let mut stream = ArgStream::new(strings);
		self.parse_argstream(&mut stream)?;
		Ok(())
	}

	fn parse_argstream(&mut self, argstream: &mut ArgStream) -> Result<(), Error> {
		let mut is_first_arg = true;

		while argstream.has_next() {
			match argstream.next().as_str() {
				"--" => {
					while argstream.has_next() {
						self.args.push(argstream.next());
					}
				}
				arg if arg.starts_with("--") => match arg.contains("=") {
					true => self.handle_equals_opt(&arg)?,
					false => self.handle_long_opt(&arg, argstream)?,
				},
				arg if arg.starts_with("-") => match arg {
					"-" => self.args.push(arg.to_string()),
					a if a.contains("=") => self.handle_equals_opt(arg)?,
					_ => self.handle_short_opt(&arg, argstream)?,
				},
				arg if is_first_arg && self.command_map.contains_key(arg) => {
					let mut cp = self.commands.remove(*self.command_map.get(arg).unwrap());
					self.command_map.clear();
					self.commands.clear();

					cp.parse_argstream(argstream)?;
					// if let Some(callback) = cp.callback {
					// 	callback(&arg, &cp);
					// }
					self.cmd_name = Some(arg.to_string());
					self.cmd_parser = Some(Box::new(cp));
				}
				"help" if is_first_arg && self.cmd_help => {
					if !argstream.has_next() {
						return Err(Error::MissingHelpArg);
					}

					let name = argstream.next();
					match self.command_map.get(&name) {
						Some(i) => {
							let cp = &mut self.commands[*i];
							let helptext = cp.helptext.as_deref().unwrap_or("").trim();
							println!("{}", helptext);
							std::process::exit(0);
						}
						None => {
							return Err(Error::InvalidName(format!(
								"'{}' is not a recognised command name",
								&name
							)));
						}
					}
				}
				arg => self.args.push(arg.to_string()),
			}

			is_first_arg = false;
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

		if arg == "--help" && self.helptext.is_some() {
			println!("{}", self.helptext.as_ref().unwrap().trim());
			std::process::exit(0);
		}
		if arg == "--version" && self.version.is_some() {
			println!("{}", self.version.as_ref().unwrap().trim());
			std::process::exit(0);
		}
		return Err(Error::InvalidName(format!(
			"{arg} is not a recognised flag or option name",
		)));
	}

	fn handle_short_opt(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
		for c in arg.chars().skip(1) {
			if let Some(index) = self.flag_map.get(&c.to_string()) {
				self.flags[*index].count += 1;
			} else if let Some(index) = self.option_map.get(&c.to_string()) {
				if !argstream.has_next() {
					let msg = if arg.chars().count() > 2 {
						format!("missing value for '{}' in {}", c, arg)
					} else {
						format!("missing value for {}", arg)
					};
					return Err(Error::MissingValue(msg));
				}

				self.options[*index].values.push(argstream.next());
			} else if c == 'h' && self.helptext.is_some() {
				println!("{}", self.helptext.as_ref().unwrap().trim());
				std::process::exit(0);
			} else if c == 'v' && self.version.is_some() {
				println!("{}", self.version.as_ref().unwrap().trim());
				std::process::exit(0);
			} else {
				let msg = match arg.chars().count() > 2 {
					true => format!("'{}' in {} is not a recognised flag or option name", c, arg),
					false => format!("{} is not a recognised flag or option name", arg),
				};
				return Err(Error::InvalidName(msg));
			}
		}
		Ok(())
	}

	fn handle_equals_opt(&mut self, arg: &str) -> Result<(), Error> {
		let splits: Vec<&str> = arg.splitn(2, '=').collect();
		let name = splits[0];
		let value = splits[1];

		if let Some(index) = self.option_map.get(name.trim_start_matches('-')) {
			if value == "" {
				return Err(Error::MissingValue(format!("missing value for {}", name)));
			}

			self.options[*index].values.push(value.to_string());
			return Ok(());
		}
		return Err(Error::InvalidName(format!(
			"{name} is not a recognised option name"
		)));
	}
}

// This type functions as a wrapper to make the input argument vector available as a stream.
struct ArgStream {
	args: Vec<String>,
	index: usize,
}

impl ArgStream {
	fn new(args: Vec<String>) -> ArgStream {
		ArgStream {
			args: args,
			index: 0,
		}
	}

	fn has_next(&self) -> bool {
		self.index < self.args.len()
	}

	fn next(&mut self) -> String {
		self.index += 1;
		self.args[self.index - 1].clone()
	}
}

// We create a single Opt instance for each registered option, i.e. each call to `.option()`.
#[derive(Debug)]
struct Opt {
	values: Vec<String>,
	default: String,
}

// We create a single Flag instance for each registered flag, i.e. each call to `.flag()`.
#[derive(Debug)]
struct Flag {
	count: usize,
}
