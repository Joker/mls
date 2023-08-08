use crate::ext::flarg::ArgParser;

#[derive(Debug, Clone)]
pub struct Flags {
	pub all: bool,
	pub one: bool,
	pub long: bool,
	pub size_sort: bool,
	pub time_sort: bool,
	pub name_sort: bool,
	pub full: bool,
	pub follow: bool,
	pub bytes: bool,
	pub ctime: bool,
	pub access: bool,
	pub create: bool,
	pub dir_only: bool,
	pub group: bool,
	pub octal: bool,
	pub inode: bool,
	pub xattr: bool,
	pub lvl: usize,
	pub list_format: bool,
	pub tree_format: bool,
}

pub fn args_init() -> (Flags, Vec<String>) {
	let mut parser = ArgParser::new()
		.helptext("USAGE:\n\tmls [options] [--] [file ...]\nOPTIONS:")
		.flag_with("a", "Include directory entries whose names begin with a dot (`.`).")
		.flag_with("d", "List of directories only.")
		.flag_with("1", "List files one entry per line.")
		.flag_with("C", "List files in the multi-column format. (default)")
		.flag_with("l", "List files in the long format.")
		.flag_with("S", "Sort by size. \t\t\t\t\t\t\t(long format)")
		.flag_with("t", "Sort by time. \t\t\t\t\t\t\t(long format)")
		.flag_with("N", "Sort by name. \t\t\t\t\t\t\t(all formats)")
		.flag_with("u", "Use time of last access. \t\t\t\t\t(long format)")
		.flag_with("U", "Use time when file was created. \t\t\t\t(long format)")
		.flag_with("c", "Use time when file status was last changed. \t\t\t(long format)")
		.flag_with("g", "Display file group name. \t\t\t\t\t(long format)")
		.flag_with("b h", "Display file sizes in bytes. \t\t\t\t\t(long format)")
		.flag_with("i", "Display file serial number <inode>. \t\t\t\t(long format)")
		.flag_with("O", "Display file permission in octal format. \t\t\t(long format)")
		.flag_with("@", "Display file extended attributes. \t\t\t\t(long format)")
		.flag_with("L", "Display file information pointed by the symbolic link. \t(long format)")
		.flag_with("f", "Display absolute path for symbolic link. \t\t\t(long format)")
		.flag_with("help", "Display list of command-line options.")
		.flag_with("2", "Recurse into directories as a tree. Limit the depth 2.")
		.flag_with("3", "Recurse into directories as a tree. Limit the depth 3.")
		.option_with("T", "9", "Recurse into directories as a tree. DEPTH - limit the depth of recursion.");

	if let Err(err) = parser.parse() {
		err.exit();
	}
	if parser.found("help") {
		parser.print_help();
	}

	let tree = (parser.found("2"), parser.found("3"), parser.found("T"));
	let lvl = if tree.0 {
		2
	} else if tree.1 {
		3
	} else {
		parser.value("T").parse::<usize>().unwrap_or(9)
	};

	let mut fl = Flags {
		all: parser.found("a"),
		one: parser.found("1"),
		long: parser.found("l"),
		size_sort: parser.found("S"),
		time_sort: parser.found("t"),
		name_sort: parser.found("N"),
		full: parser.found("f"),
		follow: parser.found("L"),
		bytes: parser.found("b"),
		ctime: parser.found("c"),
		access: parser.found("u"),
		create: parser.found("U"),
		dir_only: parser.found("d"),
		group: parser.found("g"),
		inode: parser.found("i"),
		xattr: parser.found("@"),
		octal: parser.found("O"),
		lvl,
		list_format: false,
		tree_format: tree.0 || tree.1 || tree.2,
	};
	match &parser.app_path {
		Some(p) => match p.rsplit('/').next().unwrap_or("") {
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

	fl.list_format =
		fl.long
			|| fl.size_sort
			|| fl.time_sort
			|| fl.inode || fl.group
			|| fl.xattr || fl.bytes
			|| fl.octal || fl.follow
			|| fl.ctime || fl.access
			|| fl.create || fl.full;

	if parser.found("C") || parser.found("1") {
		fl.list_format = false;
		fl.size_sort = false;
		fl.time_sort = false;
	}

	let dirs = if !parser.args.is_empty() { parser.args } else { vec![".".to_string()] };
	(fl, dirs)
}
