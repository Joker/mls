use crate::ext::flarg::ArgParser;

pub const TREE_LEVEL: usize = 2;

#[derive(Debug)]
pub struct Flags {
	pub all: bool,
	pub long: bool,
	pub Size_sort: bool,
	pub time_sort: bool,
	pub full: bool,
	pub bytes: bool,
	pub ctime: bool,
	pub u_access: bool,
	pub U_create: bool,
	pub dir_only: bool,
	pub group: bool,
	pub lvl: usize,
	pub list_format: bool,
	pub tree_format: bool,
	pub xattr: bool,
}

pub fn args_init() -> (Flags, Vec<String>) {
	let mut parser = ArgParser::new()
		.helptext("USAGE:\n\tls [options] [file ...]\nOPTIONS:")
		.flag_with("a", "Include directory entries whose names begin with a dot (`.`).")
		.flag_with("l", "List files in the long format.")
		.flag_with("d", "List of directories only.")
		.flag_with("S", "Sort by size.")
		.flag_with("t", "Sort by time.")
		.flag_with("u", "Use time of last access.")
		.flag_with("U", "Use time when file was created.")
		.flag_with("c", "Use time when file status was last changed.")
		.flag_with("g", "Display the group name. (long format)")
		.flag_with("b h", "Display file sizes in bytes. (long format)")
		.flag_with("@", "Display extended attributes. (long format)")
		.flag_with("f", "Display absolute path for symbolic link. (long format)")
		.flag_with("help", "Display list of command-line options.")
		.flag_with("T", "Recurse into directories as a tree.")
		.flag_with("2", "Recurse into directories as a tree. Limit the depth 2.")
		.flag_with("3", "Recurse into directories as a tree. Limit the depth 3.")
		.option_with(
			"L",
			TREE_LEVEL.to_string().as_str(),
			"Recurse into directories as a tree. DEPTH - limit the depth of recursion.",
		);

	if let Err(err) = parser.parse() {
		err.exit();
	}
	if parser.found("help") {
		parser.print_help();
	}

	let tree = (
		parser.found("2"),
		parser.found("3"),
		parser.found("T"),
		parser.found("L"),
	);
	let lvl = if tree.0 {
		2
	} else if tree.1 {
		3
	} else if tree.2 {
		999
	} else if tree.3 {
		parser.value("L").parse::<usize>().unwrap_or(TREE_LEVEL)
	} else {
		TREE_LEVEL
	};

	let mut fl = Flags {
		all: parser.found("a"),
		long: parser.found("l"),
		Size_sort: parser.found("S"),
		time_sort: parser.found("t"),
		full: parser.found("f"),
		bytes: parser.found("b"),
		ctime: parser.found("c"),
		u_access: parser.found("u"),
		U_create: parser.found("U"),
		dir_only: parser.found("d"),
		group: parser.found("g"),
		xattr: parser.found("@"),
		lvl,
		list_format: false,
		tree_format: tree.0 || tree.1 || tree.2 || tree.3,
	};
	match &parser.app_path {
		Some(p) => match p.rsplit("/").next().unwrap_or("") {
			"ll" => fl.long = true,
			"la" => fl.all = true,
			"lla" | "lal" => {
				fl.long = true;
				fl.all = true;
			}
			"lt" | "tree" => fl.tree_format = true,
			"ltl" | "llt" => {
				fl.tree_format = true;
				fl.long = true;
			}
			"lsd" => fl.dir_only = true,
			_ => (),
		},
		None => (),
	};

	fl.list_format = fl.long || fl.Size_sort || fl.time_sort || fl.group || fl.xattr;

	let dirs = if parser.args.len() > 0 { parser.args } else { vec![".".to_string()] };
	(fl, dirs)
}
