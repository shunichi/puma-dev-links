use std::fs;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::collections::HashSet;
use dirs;
mod options;
use options::SubCommand;

enum EntryType {
    Link { target: String },
    Port { port: i32 },
    Invalid,
}

struct Entry {
    name: String,
    entry_type: EntryType,
}

fn entry_type_ord(entry: &Entry) -> i32 {
    match entry.entry_type {
        EntryType::Link { .. } => 0,
        EntryType::Port { .. } => 1,
        EntryType::Invalid => 2,
    }
}

fn entry_cmp(e1: &Entry, e2: &Entry) -> Ordering {
    let o1 = entry_type_ord(e1);
    let o2 = entry_type_ord(e2);
    let ord = o1.cmp(&o2);
    if ord != Ordering::Equal {
        return ord;
    }
    match e1.entry_type {
        EntryType::Port { port, .. } => {
            let port1 = port;
            match e2.entry_type {
                EntryType::Port { port, .. } => {
                    port1.cmp(&port)
                },
                _ => panic!()
            }
        },
        _ => e1.name.cmp(&e2.name)
    }
}

fn puma_dev_dir() -> Option<PathBuf> {
    let mut dir_path = dirs::home_dir()?;
    dir_path.push(".puma-dev");
    Some(dir_path)
}

fn current_dir_basename() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    cwd.file_name().and_then(|name| name.to_str().map(|s| s.to_owned()))
}

fn get_puma_dev_entries() -> Option<Vec<Entry>> {
    let dir_path = puma_dev_dir()?;
    let mut vec = Vec::new();
    for dir_entry in  fs::read_dir(dir_path).ok()? {
        let dir = dir_entry.ok()?;
        let file_type = dir.file_type().ok()?;
        let file_name = dir.file_name().to_string_lossy().to_string();
        if file_type.is_symlink() {
            let target_path = fs::read_link(dir.path()).ok()?;
            vec.push(Entry { name: file_name, entry_type: EntryType::Link { target: target_path.to_string_lossy().to_string() } });
        } else if file_type.is_file() {
            let content = fs::read_to_string(dir.path()).ok()?;
            let first_line = content.lines().next().unwrap_or("");
            match first_line.parse::<i32>() {
                Ok(port) => {
                    vec.push(Entry { name: file_name, entry_type: EntryType::Port { port: port } });
                },
                Err(_) => {
                    vec.push(Entry { name: file_name, entry_type: EntryType::Invalid });
                }
            }
        } else {
            vec.push(Entry { name: file_name, entry_type: EntryType::Invalid });
        }
    }
    vec.sort_by(|a, b| entry_cmp(a, b ));
    Some(vec)
}

fn next_port() -> Option<i32> {
    let entries = get_puma_dev_entries()?;
    let set: HashSet<_> = entries.iter().filter_map(|e|
        match e.entry_type {
            EntryType::Port { port } => Some(port),
            _ => None,
        }
    ).collect();
    let mut port = 3000;
    loop {
        if !set.contains(&port) {
            return Some(port);
        }
        port += 1;
    }
}

fn app_entry_path(option_app_name: Option<String>) -> Option<(String, PathBuf)> {
    let mut path = puma_dev_dir()?;
    let app_name = match option_app_name {
        Some(n) => n,
        None => current_dir_basename()?,
    };
    path.push(&app_name);
    Some((app_name, path))
}

fn list_entries() -> Option<()> {
    let entries = get_puma_dev_entries()?;
    if entries.len() == 0 {
        return Some(());
    }

    let name_width = entries.iter().map(|e| e.name.len()).max()?;
    for entry in entries {
        match entry.entry_type {
            EntryType::Link { target } => {
                println!("{:width$} -> {}", entry.name, target, width = name_width);
            },
            EntryType::Port { port } => {
                println!("{:width$} {}", entry.name, port, width = name_width);
            }
            EntryType::Invalid => {
                println!("{:width$} invalid", entry.name,  width = name_width);
            }
        }
    }
    Some(())
}

fn show_port(option_app_name: Option<String>) -> Option<()> {
    let (app_name, _) = app_entry_path(option_app_name)?;
    let entries = get_puma_dev_entries()?;
    match entries.iter().find(|e| e.name == app_name) {
        Some(entry) => {
            match &entry.entry_type {
                EntryType::Port { port } => {
                    print!("{}", port);
                },
                EntryType::Link { target } =>  {
                    eprintln!("error: '{}' is symlink to '{}'", app_name, target);
                    return None;
                },
                EntryType::Invalid =>  {
                    eprintln!("error: '{}' is invalid entry", app_name);
                    return None;
                },
            }
        },
        None => {
            eprintln!("error: can't find app '{}'", app_name);
            return None;
        }
    }
    Some(())
}

fn link_app(option_app_name: Option<String>) -> Option<()> {
    let (app_name, path) = app_entry_path(option_app_name)?;
    if path.exists(){
        eprintln!("error: '{}' already exists", path.to_string_lossy());
        return Some(());
    }
    let port = next_port().map(|p| p.to_string())?;
    fs::write(path, &port).ok()?;
    println!("'{}' is linked to port {}", app_name, port);
    Some(())
}

fn unlink_app(option_app_name: Option<String>) -> Option<()> {
    let (app_name, path) = app_entry_path(option_app_name)?;
    if !path.exists(){
        eprintln!("error: app '{}' does not exists", app_name);
        return Some(());
    }
    let meta = fs::symlink_metadata(&path).ok()?;
    let file_type = meta.file_type();
    if file_type.is_symlink() {
        fs::remove_file(path).unwrap();
    } else if file_type.is_file() {
        fs::remove_file(path).unwrap();
    } else if file_type.is_dir() {
        eprintln!("error: '{}' is a directory", app_name);
        return None;
    }

    println!("'{}' is unlinked", app_name);
    Some(())
}

fn main() -> () {
    let options = options::parse_opts();
    let result = match options.sub_command {
        SubCommand::List => { list_entries() },
        SubCommand::Port { app_name } => { show_port(app_name) },
        SubCommand::Link { app_name } => { link_app(app_name) },
        SubCommand::Unlink { app_name } => { unlink_app(app_name) },
    };
    match result {
        Some(_) => std::process::exit(0),
        None => std::process::exit(1),
    };
}
