use anyhow::{format_err, Result};
use luafmt_lib::{format_code, Config};
use std::fs;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "luafmt", about = "A utility to format Lua code")]
struct Opt {
    #[structopt(long = "config-path", parse(from_os_str))]
    config_path: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn format(opt: Opt) -> Result<i32> {
    if opt.files.is_empty() {
        return Err(format_err!("error: no files provided"));
    }

    let config: Config = match opt.config_path {
        Some(config_path) => match fs::read_to_string(config_path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format_err!("error: config file not in correct format: {}", error));
                }
            },
            Err(error) => {
                return Err(format_err!("error: couldn't read config file: {}", error));
            }
        }

        None => match fs::read_to_string("luafmt.toml") {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format_err!("error: config file not in correct format: {}", error));
                }
            }

            Err(_) => Config::default()
        }
    };

    let mut errors = vec![];

    for file_path in opt.files.iter() {
        if file_path.exists() {
            let mut file = match fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path)
            {
                Ok(file) => file,
                Err(error) => {
                    errors.push(format_err!(
                        "error: could not open file {}: {}",
                        file_path.display(),
                        error
                    ));
                    continue;
                }
            };

            let mut contents_buffer = Vec::new();
            if let Err(err) = file.read_to_end(&mut contents_buffer) {
                errors.push(format_err!(
                    "error: couldn't read contents of file {}: {}",
                    file_path.display(),
                    err
                ));
                continue;
            }

            let contents = String::from_utf8_lossy(&contents_buffer);
            let formatted_contents = match format_code(&contents, config) {
                Ok(formatted) => formatted,
                Err(error) => {
                    errors.push(format_err!(
                        "error: could not format file {}: {}",
                        file_path.display(),
                        error
                    ));

                    continue;
                }
            };

            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            match file.write_all(formatted_contents.as_bytes()) {
                Ok(_) => (),
                Err(error) => {
                    errors.push(format_err!(
                        "error: could not write to file {}: {}",
                        file_path.display(),
                        error
                    ));
                    
                    continue;
                }
            }
        } else {
            errors.push(format_err!("error: file {} not found", file_path.display()));
            continue;
        }
    }

    if errors.len() > 0 {
        for error in errors.iter() {
            eprintln!("{}", error.to_string());
        }
        return Ok(1)
    }

    Ok(0)
}

fn main() {
    let opt = Opt::from_args();

    let exit_code = match format(opt) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e.to_string());
            1
        }
    };

    std::process::exit(exit_code);
}
