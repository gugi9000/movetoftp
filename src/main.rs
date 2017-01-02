extern crate ftp;
#[macro_use]
extern crate clap;

use std::fs::{self, File};
use std::path::PathBuf;

use ftp::FtpStream;
use ftp::types::FileType;

use clap::{App, Arg, AppSettings};

fn main() {
     let m = App::new(env!("CARGO_PKG_NAME"))
        .author(crate_authors!())
        .version(crate_version!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ColoredHelp)
        .args(&[
            Arg::with_name("from")
                 .help("The path to the local folder where the files will be moved from")
                 .short("f")
                 .long("from")
                 .takes_value(true)
                 .value_name("PATH")
                 .default_value("."),
            Arg::with_name("server")
                .short("s")
                .long("server")
                .takes_value(true)
                .value_name("HOST")
                .required(true)
                .help("The hostname of the FTP-server"),
            Arg::with_name("port")
                .short("p")
                .validator(|p| p.parse::<u16>().map(|_| ()).map_err(|_| "Not a valid port".to_owned()))
                .long("port")
                .takes_value(true)
                .value_name("PORT")
                .default_value("21")
                .help("The hostname of the FTP-server"),
            Arg::with_name("to")
                 .short("t")
                 .long("to")
                 .takes_value(true)
                 .value_name("PATH")
                 .help("The remote path on FTP-server where the files will be moved to"),
            Arg::with_name("username")
                 .short("u")
                 .long("username")
                 .takes_value(true)
                 .value_name("USERNAME")
                 .required(true)
                 .help("The username of the user on the FTP-server to login with"),
            Arg::with_name("password")
                 .short("P")
                 .long("password")
                 .takes_value(true)
                 .value_name("PASSWORD")
                 .required(true)
                 .help("The password of the user on the FTP-server to login with"),
            Arg::with_name("delete")
                 .short("d")
                 .long("delete")
                 .help("Deletes emptied folders after moving files"),
        ])
        .get_matches();

    let local_path = m.value_of("from").unwrap().into();
    let hostname = m.value_of("server").unwrap();
    let port_number: u16 = m.value_of("port").and_then(|s| s.parse().ok()).unwrap();
    let remote_path = m.value_of("to");
    let username = m.value_of("username").unwrap();
    let password = m.value_of("password").unwrap();
    let delete_folders = m.is_present("delete");

    println!("Connecting..");

    let mut ftp_stream = match FtpStream::connect((&*hostname, port_number)){
        Ok(s) => s,
        Err(e) => return println!("Failed to connect to host:\n\t{}", e)
    };
    match ftp_stream.login(&username, &password){
        Ok(()) => (),
        Err(e) => return println!("Failed to login:\n\t{}", e)
    };
    ftp_stream.transfer_type(FileType::Binary).unwrap();
    if let Some(ref p) = remote_path{
        match ftp_stream.cwd(p){
            Ok(()) => (),
            Err(e) => {
                println!("Error happened setting the remote path:\n\t{}", e);
                return
            }
        }
    }

    let mut errors = 0;
    put_files(&mut ftp_stream, local_path, "./".into(), &mut errors, delete_folders);

    ftp_stream.quit().unwrap();

    println!("Finished with {} error{}",
    errors,
    match errors{
        // Smiley when there weren't any errors :)
        0 => "s :)",
        1 => "!",
        _ => "s!"
    });
}

use std::fmt::Display;

fn error(errors: &mut usize, action: &str, e: &Display){
    println!("\tError {} file:\n\t\t{}", action, e);
    *errors += 1;
}

use std::borrow::Cow;

fn put_files(stream: &mut FtpStream, dir: PathBuf, folder: Cow<str>, errors: &mut usize, delete: bool){
    if folder != "./"{
        match stream.mkdir(&folder){
            Ok(_) => (),
            Err(e) => println!("Error happened making remote folder ({}):\n\t{}", folder, e)
        }
    }
    for entry in dir.read_dir().unwrap(){
        let entry = entry.unwrap();

        match entry.file_type().unwrap(){
            t if t.is_file() => {
                let remote_file = &format!("{}/{}", folder, entry.file_name().to_string_lossy());
                let file = entry.path();

                println!("Uploading {} to {}", file.display(), remote_file);

                match File::open(file) {
                    Ok(ref mut f) => match stream.put(remote_file, f){
                        Ok(()) => match fs::remove_file(entry.path()){
                            Ok(()) => println!("\tSuccess deleting local file"),
                            Err(e) => error(errors, "deleting", &e)
                        },
                        Err(e) => error(errors, "putting", &e)
                    },
                    Err(e) => error(errors, "opening", &e)
                }
            },
            t if t.is_dir() => put_files(stream, entry.path(), entry.file_name().to_string_lossy(), errors, delete),
            _ => ()
        }
    }
    if delete && folder != "./"{
        match fs::remove_dir(&dir){
            Ok(()) => (),
            Err(_) => println!("Couldn't remove folder {}", dir.display())
        }
    }
}
