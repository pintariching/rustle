use std::{fs::{self, ReadDir}, path::{Path, PathBuf}, io::ErrorKind, env::current_dir};


use rustle::compile_file_to_js;
use clap::{Parser, Subcommand};
use console::style;


#[derive(Parser)]
#[command(name = "Rustle")]
#[command(version = "0.1.0")]
#[command(about = "Svelte compiler rewritten in Rust", long_about = None)]
struct Cli {
    file: Option<String>,

    #[arg(short = 'o', long = "output")]
    output: Option<String>,

    #[arg(short = 'y', long = "yes")]
    yes: Option<String>,

    #[command(subcommand)]
    command: Option<Sub>
}


#[derive(Subcommand)]
enum Sub {
    parse {
        file: Option<String>
    }
}


fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.file.as_deref() {
        let res = match fs::metadata(file) {
            Ok(md) => {
                if md.is_dir() {
                    let files = match fs::read_dir(file) {

                        Ok(rd) => {
                            let mut output = current_dir().unwrap();
                            output.push("./dist");
                            let dir_creation_result = fs::create_dir(output.clone());


                            if let Ok(()) = dir_creation_result {
                                compile_directory(rd, output.clone())
                            } else {
                                let e = dir_creation_result.err().unwrap();

                                if let ErrorKind::AlreadyExists = e.kind() {
                                    compile_directory(rd, output.clone());
                                } else {
                                    println!("{}: {}", style("[ERROR]").red(), e);
                                }
                            }

                        },
                        Err(e) => {
                            println!("{}: {}", style("[ERROR]").red(), e);
                        }
                    };
                } else if md.is_file() {
                    if let Some(out_file) = cli.output.as_deref() {
                        let mut location = current_dir().unwrap();
                        let mut input = location.clone();
                        let filePath = Path::new(file);
                        input.push(filePath);


                        location.push(out_file);

                        let output = compile_file_to_js(input.as_path(), location.as_path());

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    } else {
                        let mut location = current_dir().unwrap();
                        let mut input = location.clone();
                        let filePath = Path::new(file);
                        input.push(filePath);


                        location.push(filePath.file_stem().unwrap().to_str().unwrap().to_owned() + ".js");

                        let output = compile_file_to_js(input.as_path(), location.as_path());

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    }
                }
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("{}: File or Directory not found", style("[ERROR]").red());
                    },

                    ErrorKind::PermissionDenied => {
                        println!("{}: Permission Denied", style("[ERROR]").red());
                    },

                    _ => {
                        println!("{}: unexpected error", style("[ERROR]").red());
                    }
                }
            }
        };

    }


    // Parse subcommand
    match &cli.command {
        Some(Sub::parse { file}) => {
            if let Some(file) = file.as_deref() {
                let res = match fs::metadata(file) {
                    Ok(md) => {

                        // Implement a parse option that outputs json
                        todo!()
                    },
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::NotFound => {
                                println!("{}: File or Directory not found", style("[ERROR]").red());
                            },
        
                            ErrorKind::PermissionDenied => {
                                println!("{}: Permission Denied", style("[ERROR]").red());
                            },
        
                            _ => {
                                println!("{}: unexpected error", style("[ERROR]").red());
                            }
                        }
                    }
                };
            }
        },
        None => {}
    }
}


fn compile_directory(rd: ReadDir, output: PathBuf) {
    for i in rd {
        match i {
            Ok(de) => {
                let location = de.path();

                if let Some(ext) = location.extension() {
                    if ext == "rustle" || ext == "svelte" {
                        let mut outfilelocation = output.clone();
                        outfilelocation.push(location.file_stem().unwrap().to_str().unwrap().to_owned()+ ".js");


                        let output = compile_file_to_js(&location, &outfilelocation);

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    } else {

                        //Maybe dont print this? 
                        println!("{}: {} doesnt end with .svelte or .rustle", style("[WARNING]").yellow(), de.file_name().to_str().unwrap());
                    }
                }

            
            },
            Err(e) => {
                println!("{}: {}", style("[ERROR]").red(), e);
            }
        }
    }   
}
