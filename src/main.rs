extern crate ftp;
extern crate getopts;

use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

use ftp::FtpStream;
use ftp::types::FileType;

use getopts::Options;

macro_rules! required {
    ($opt:expr) => (
        if let Some(x) = $opt{
            x
        }else{
            println!("A required arguement was missing!");
            return incorrect_syntax()
        }
    );
}

fn incorrect_syntax(){
    println!("Incorrect syntax.\nTry -h for help");
}
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    println!("Move to FTP v{} © 2016 LFalch.com\n", env!("CARGO_PKG_VERSION"));

    let mut opts = Options::new();
    opts.optopt("f", "from", "set the path to the local folder where the files will be moved from (default is current working directory)", "PATH");
    opts.optopt("s", "server", "set the hostname of the FTP-server (required)", "HOST");
    opts.optopt("p", "port", "set the port on the FTP-server (default: 21)", "PORT");
    opts.optopt("t", "to", "set the remote path on the FTP-server where the files will be moved to", "PATH");
    opts.optopt("u", "username", "set the username of the user on the FTP-server to login with (required)", "USERNAME");
    opts.optopt("P", "password", "set the password of the user on the FTP-server to login with (required)", "PASSWORD");
    opts.optflag("d", "delete", "deletes emptied folders after moving files");
    opts.optflag("h", "help", "prints this help");

    let matches = match opts.parse(&args){
        Ok(m) => m,
        Err(_) => return incorrect_syntax()
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage(""));
        return
    }

    let local_path = PathBuf::from(matches.opt_str("f").unwrap_or_else(|| ".".to_owned()));
    let hostname = required!(matches.opt_str("s"));
    let port_number: u16 = match matches.opt_str("p").ok_or(()){
        // default port
        Err(()) => 21,
        Ok(p) => match p.parse(){
            Ok(p) => p,
            Err(e) => return
                println!("Error parsing port as number:\n\t{}\nDid you type a real number?", e)
        }
    };
    let remote_path = matches.opt_str("t");
    let username = required!(matches.opt_str("u"));
    let password = required!(matches.opt_str("P"));
    let delete_folders = matches.opt_present("d");

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

use std::borrow::Cow;

fn put_files(stream: &mut FtpStream, dir: PathBuf, folder: Cow<str>, errors: &mut usize, delete: bool){
    if folder != "./"{
        match stream.mkdir(&folder){
            Ok(_) => (),
            Err(e) => {
                println!("Error happened making remote folder ({}):\n\t{}", folder, e);
            }
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
                        Ok(()) => {
                            match fs::remove_file(entry.path()){
                                Ok(()) => println!("\tSuccess deleting local file"),
                                Err(e) => {
                                    println!("\tError deleting file:\n\t\t{}", e);
                                    *errors += 1;
                                }
                            }
                        },
                        Err(e) => {
                            println!("\tError putting file:\n\t\t{}", e);
                            *errors += 1;
                        }
                    },
                    Err(e) => {
                        println!("\tError opening file:\n\t\t{}", e);
                        *errors += 1;
                    }
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
