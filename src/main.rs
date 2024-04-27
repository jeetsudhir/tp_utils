use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use structopt::StructOpt;
use std::process::{Command, Stdio};

#[derive(StructOpt, Debug)]
struct Opt {
    /// Set a new teleport point
    #[structopt(short, long)]
    set: bool,

    /// The name of the teleport point
    name: Option<String>,

    /// List all teleport points
    #[structopt(long)]
    list: bool,

    /// Remove a teleport point
    #[structopt(long)]
    delete: bool,

    /// Remove all teleport points
    #[structopt(long)]
    all: bool,
}

fn get_tp_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".tp")
}

fn set_tp_point(name: &str) -> io::Result<()> {
    let tp_dir = get_tp_dir();
    fs::create_dir_all(&tp_dir)?;
    let tp_path = tp_dir.join(name);
    let current_dir = env::current_dir()?.to_str().unwrap().to_string();
    if tp_path.exists() {
        let existing_path = fs::read_to_string(&tp_path)?;
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Teleport point '{}' already exists and points to '{}'", name, existing_path.trim())));
    }
    for entry in fs::read_dir(&tp_dir)? {
        let entry = entry?;
        let path = fs::read_to_string(entry.path())?;
        if path.trim() == current_dir {
            let existing_name = entry.file_name().into_string().unwrap();
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Teleport point to '{}' already exists with the name '{}'", current_dir, existing_name)));
        }
    }
    fs::write(tp_path, current_dir)
}

fn get_tp_point(name: &str) -> io::Result<()> {
    let tp_dir = get_tp_dir();
    let tp_path = tp_dir.join(name);

    match fs::read_to_string(&tp_path) {
        Ok(contents) => {
            let trimmed_path = contents.trim(); // Bind the trimmed path to a variable
            let command = if std::env::consts::OS == "macos" {
                format!("tell application \"Terminal\" to do script \"cd {}\"", trimmed_path)
            } else {
                format!("cd {}", trimmed_path)
            };

            let mut child = Command::new(if std::env::consts::OS == "macos" {
                "osascript"
            } else {
                "x-terminal-emulator"
            })
            .arg("-e")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

            match child.wait() {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            }
        }
        Err(_) => {
            eprintln!("Teleport point '{}' not found", name);
            Err(io::Error::new(io::ErrorKind::NotFound, "Teleport point not found"))
        }
    }
}

fn list_tp_points() -> io::Result<()> {
    let tp_dir = get_tp_dir();
    if !tp_dir.exists() {
        println!("No teleport points have been made. You can make a teleport point with the following command:\n\ncargo run -- --set --name <name>");
        return Ok(());
    }
    for entry in fs::read_dir(tp_dir)? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        let path = fs::read_to_string(entry.path())?;
        println!("{}: {}", name, path);
    }
    Ok(())
}

fn delete_tp_point(name: Option<String>, all: bool) -> io::Result<()> {
    let tp_dir = get_tp_dir();
    if all {
        fs::remove_dir_all(tp_dir)?;
    } else {
        match name {
            Some(name) => {
                let tp_path = tp_dir.join(name);
                fs::remove_file(tp_path)?;
            }
            None => eprintln!("You must provide a name for the teleport point to delete"),
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    if opt.list {
        list_tp_points()?;
    } else if opt.delete {
        delete_tp_point(opt.name, opt.all)?;
    } else {
        match opt.name {
            Some(name) => {
                if opt.set {
                    set_tp_point(&name)?;
                } else {
                    get_tp_point(&name)?;
                }
            }
            None => eprintln!("You must provide a name for the teleport point"),
        }
    }

    Ok(())
}
