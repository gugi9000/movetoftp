extern crate ftp;
extern crate getopts;

use std::env;
use std::fs::{self, File};

use ftp::FtpStream;
use ftp::types::FileType;

use getopts::Options;

macro_rules! required {
    ($opt:expr) => (
        if let Some(x) = $opt{
            x
        }else{
            println!("A required arguement was missing!");
            incorrect_syntax();
            return
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
    opts.optopt("P", "password", "set the password of the user on the FTP-server to login with (required)", "PASSWORD");
    opts.optopt("u", "username", "set the username of the user on the FTP-server to login with (required)", "USERNAME");
    opts.optflag("h", "help", "prints this help");

    let matches = match opts.parse(&args){
        Ok(m) => m,
        Err(_) => return incorrect_syntax()
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage(""));
        return
    }

    let local_path = fs::read_dir(matches.opt_str("f").unwrap_or_else(|| ".".to_owned())).unwrap();
    let hostname = required!(matches.opt_str("s"));
    let port_number: u16 = matches.opt_str("p").map(|p| p.parse().unwrap()).unwrap_or(21);
    let remote_path = matches.opt_str("t");
    let username = required!(matches.opt_str("u"));
    let password = required!(matches.opt_str("P"));

    println!("Connecting..");

    let mut ftp_stream = FtpStream::connect((&*hostname, port_number)).unwrap();
    ftp_stream.login(&username, &password).unwrap();
    ftp_stream.transfer_type(FileType::Binary).unwrap();
    if let Some(ref p) = remote_path{
        ftp_stream.cwd(p).unwrap();
    }

    put_files(&mut ftp_stream, local_path, "./".to_owned());

    ftp_stream.quit().unwrap();

    println!("Finished!")
}

fn put_files(stream: &mut FtpStream, dir: fs::ReadDir, folder: String){
    if folder != "./"{
        stream.mkdir(&folder).unwrap()
    }
    for entry in dir{
        let entry = entry.unwrap();

        match entry.file_type().unwrap(){
            t if t.is_file() => {
                let remote_file = &format!("{}/{}", folder, entry.path().file_name().unwrap().to_str().unwrap());
                let file = entry.path();

                println!("Uploading {} to {}", file.display(), remote_file);

                match stream.put(remote_file, &mut File::open(file).unwrap()){
                    Ok(()) => {
                        println!("\tSuccess, deleting local file");
                        fs::remove_file(entry.path()).unwrap();
                    },
                    Err(e) => {
                        println!("\tError happened: {}", e);
                    }
                }
            },
            t if t.is_dir() => put_files(stream, fs::read_dir(entry.path()).unwrap(), entry.path().file_name().unwrap().to_string_lossy().into_owned()),
            _ => ()
        }
    }

}
