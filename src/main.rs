use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use std::process::{Command, Stdio};

#[derive(StructOpt, Debug)]
#[structopt(name = "tp_utils", about = "A teleport command for the terminal.")]
enum Opt {
    /// Set a new teleport point
    #[structopt(name = "set")]
    Set {
        /// The name of the teleport point
        name: String,
    },

    /// List all teleport points
    #[structopt(name = "list")]
    List,

    /// Remove a teleport point or all teleport points
    #[structopt(name = "delete")]
    Delete {
        /// The name of the teleport point to delete
        name: Option<String>,

        /// Remove all teleport points
        #[structopt(short = "a", long = "all")]
        all: bool,
    },

    /// Teleport to a point
    #[structopt(name = "tp")]
    Teleport {
        /// The name of the teleport point
        name: String,
    },
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
    fs::write(&tp_path, current_dir.clone())?;
    println!("Teleport point '{}' set to '{}'", name, current_dir);
    Ok(())
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
        println!("No teleport points have been made. You can make a teleport point with the following command:\n\ntp set \"<name>\"");
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
                let tp_path = tp_dir.join(name.clone());
                println!("Deleting teleport point '{}'", name);
                fs::remove_file(tp_path)?;
            }
            None => eprintln!("You must provide a name for the teleport point to delete"),
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Set { name } => set_tp_point(&name.trim())?,
        Opt::List => list_tp_points()?,
        Opt::Delete { name, all } => delete_tp_point(name.map(|n| n.trim().to_string()), all)?,
        Opt::Teleport { name } => get_tp_point(&name.trim())?,
    }
    Ok(())
}
