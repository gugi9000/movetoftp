extern crate ftp;

use std::env;
use std::fs::{self, File};

use ftp::FtpStream;
use ftp::types::FileType;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    println!("Move to FTP v.0.1 © 2016 LFalch.com");
    println!("=================");

    if args.len() != 6 {
        println!("Usage: move-to-ftp.exe <local path> <ftpserver> <port> <remote path> <username> <password>.");
        println!(" ");
        println!("Example: move-to-ftp.exe c:\\Temp\\* ftp.example.com 21 /upload/ joe secret");
        println!(" ");
        println!("Files from local path will be recursively uploaded to FTP-server in remote path and then deleted locally");
        return
    }

    let local_path = fs::read_dir(&*args[0]).unwrap();
    let hostname = &*args[1];
    let port_number: u16 = args[2].parse().unwrap();
    let remote_path = &*args[3];
    let username = &*args[4];
    let password = &*args[5];

    println!("Connecting..");

    let mut ftp_stream = FtpStream::connect((hostname, port_number)).unwrap();
    ftp_stream.login(username, password).unwrap();
    ftp_stream.transfer_type(FileType::Binary).unwrap();
    ftp_stream.cwd(remote_path).unwrap();

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
