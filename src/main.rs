use clap::Parser;
use heck::ToUpperCamelCase;
use include_dir::{include_dir, Dir};
use inflector::cases::snakecase::is_snake_case;
use path_absolutize::Absolutize;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;

static TEMPLATE_DIR: Dir = include_dir!("manim_template");

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    ///Name of the project
    #[clap(long, short, default_value = "manim_template")]
    name: String,

    ///Where to create the project
    #[clap(default_value = ".")]
    path: PathBuf,
}

macro_rules! change_name {
    ($f: expr, $name: expr) => {
        let mut file = fs::File::open(&$f).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        let contents = String::from_utf8(contents).unwrap();
        let name_camel = $name.to_upper_camel_case();
        let new_contents = contents
            .replace("ManimTemplate", &name_camel)
            .replace("manim_template", &$name);

        dbg!(&new_contents);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&$f)
            .unwrap();
        file.write_all(new_contents.as_ref()).unwrap();
    };
}

fn main() {
    let Args { name, path } = Args::parse();
    let path = path.absolutize().unwrap();
    let path = path.join(&name);

    if !is_snake_case(&name) {
        eprintln!("Error: Name must follow snake case. Example: new_project");
        exit(1);
    };

    fs::create_dir(&path).unwrap();
    TEMPLATE_DIR.extract(&path).unwrap();

    change_name!(path.join("main.py"), name);
    change_name!(path.join("cfg").join("manim.cfg"), name);
    change_name!(path.join("cfg").join("manim_low.cfg"), name);
    change_name!(path.join("cfg").join("manim_very_low.cfg"), name);
}
