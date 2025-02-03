use clap::{arg, command, ArgAction, ArgMatches, Command};
use colored::Colorize;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = args_parsing_setup();

    setup_file_location()?;

    match matches.subcommand() {
        Some(("add", matches)) => add(matches)?,
        Some(("done", matches)) => done(matches)?,
        Some(("remove", _matches)) => println!("remove"),
        _ => (),
    }

    print_content()?;

    Ok(())
}

fn args_parsing_setup() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("add")
                .about("adds items to the todo list")
                .arg(arg!([ITEM]).action(ArgAction::Append))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("done")
                .about("mark as done items in the todo list")
                .arg(arg!([ITEM]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("remove items from the todo list")
                .arg(arg!([ITEM]))
                .arg_required_else_help(true),
        )
        .get_matches()
}

fn get_local_share_path() -> Result<PathBuf, Box<dyn Error>> {
    //get the system home path
    let home_path = env::var("HOME")?;
    //return the folder where the cli is going to store the todo.md list
    Ok(Path::new(&home_path).join(Path::new(".local/share/todo/")))
}

fn get_data_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let local_share_path = get_local_share_path()?;
    // return the file location for todo.md
    Ok(local_share_path.join(Path::new("todo.md")))
}

fn setup_file_location() -> Result<(), Box<dyn Error>> {
    // this function checks if the data file exists
    // and if it doesn't it create a new one
    let local_share_path = get_local_share_path()?;

    //dbg!(&local_share_path);

    match fs::exists(local_share_path.join(Path::new("todo.md"))) {
        Ok(file_exists) => {
            if !file_exists {
                dbg!("creating files...");
                fs::create_dir_all(&local_share_path)?;
                fs::write(local_share_path.join(Path::new("todo.md")), "")?;
            }
        }
        Err(e) => {
            print!("error");
            return Result::Err(Box::new(e));
        }
    }

    Ok(())
}

fn args_collect<'a>(matches: &'a ArgMatches, id: &'a str) -> Vec<&'a str> {
    // return the args collected by clap crate as a vec
    matches
        .get_many::<String>(id)
        .unwrap()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
}

fn get_data_file_content() -> Result<String, Box<dyn Error>> {
    let data_path = get_data_file_path()?;
    let file = fs::read_to_string(data_path)?;
    Ok(file)
}

fn print_content() -> Result<(), Box<dyn Error>> {
    let content = get_data_file_content()?;
    //prints the content inside the todo.md file

    content.lines().fold("", |_inc, s| {
        match &s[..=5] {
            "- [ ] " => println!("{}", &s[6..]),
            "- [x] " => println!("{}", &s[6..].strikethrough().white()),
            _ => (),
        }
        s
    });

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let items = args_collect(matches, "ITEM");
    let mut new_content: String = get_data_file_content()?;

    for item in items {
        if !new_content.contains(item) {
            new_content.push_str("- [ ] ");
            new_content.push_str(item);
            new_content.push('\n');
        }
    }

    fs::write(get_data_file_path()?, new_content)?;

    Ok(())
}

fn done(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let items = args_collect(matches, "ITEM");
    let content: String = get_data_file_content()?;
    let mut new_content: String = String::from("");
    let mut change_flag: bool = false;

    dbg!(&content);

    for line in content.lines() {
        for item in &items {
            if line.contains(&("- [ ] ".to_owned() + item).to_owned()) {
                new_content.push_str("- [x] ");
                new_content.push_str(item);
                new_content.push('\n');
                change_flag = true;
                break;
            }
        }
        if !change_flag {
            new_content.push_str(line);
            new_content.push('\n');
        }
        change_flag = false;
    }

    fs::write(get_data_file_path()?, new_content)?;

    Ok(())
}
