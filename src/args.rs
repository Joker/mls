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
}

pub fn args_init() -> (Flags, Vec<String>) {
	let mut parser = ArgParser::new()
		.helptext(
			r#"USAGE:
	ls [-alStfbcuUgd] [file ...]
OPTIONS:
	-a		Include directory entries whose names begin with a dot (`.`).
	-l		List files in the long format.
	-S		Sort by size.
	-t		Sort by time.
	-f		Absolute path for symbolic link in the list.
	-b		List file sizes in bytes.
	-c		Use time when file status was last changed.
	-u		Use time of last access, instead of time of last modification of the file.
	-U		Use time when file was created.
	-g		Display the group name.
	-d		List of directories only.
	-2		Recurse into directories as a tree. Limit the depth 2.
	-3		Recurse into directories as a tree. Limit the depth 3.
	-T		Recurse into directories as a tree.
	-L DEPTH	Recurse into directories as a tree. DEPTH - limit the depth of recursion.
	"#,
		)
		.flag("a")
		.flag("l")
		.flag("S")
		.flag("t")
		.flag("f")
		.flag("b h")
		.flag("c")
		.flag("u")
		.flag("U")
		.flag("g")
		.flag("d")
		.flag("2")
		.flag("3")
		.flag("T")
		.option("L", TREE_LEVEL.to_string().as_str());

	if let Err(err) = parser.parse() {
		err.exit();
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
		lvl,
		list_format: false,
		tree_format: false,
	};
	match &parser.app_name {
		Some(p) => match p.rsplit_once("/").unwrap_or(("","")).1 {
			"la" => fl.all = true,
			"lla" | "lal" => {
				fl.long = true;
				fl.all = true;
			}
			"ll" => fl.long = true,
			"lt" => fl.tree_format = true,
			"lsd" => fl.dir_only = true,
			_ => (),
		},
		None => (),
	};

	fl.list_format = fl.long || fl.Size_sort || fl.time_sort || fl.group;
	fl.tree_format = tree.0 || tree.1 || tree.2 || tree.3;
	
	println!("{:#?}", parser);
	let dirs = if parser.args.len() > 0 {
		parser.args
	} else {
		vec![".".to_string()]
	};
	
	(fl, dirs)
}
