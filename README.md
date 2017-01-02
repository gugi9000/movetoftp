# Move to FTP

A tool to upload local files to FTP and **delete them**, locally. Hence move to FTP :smile:

### Usage
    movetoftp.exe [FLAGS] [OPTIONS] --server <HOST> --to <PATH> --username <USERNAME> --password <PASSWORD>

#### Example
    movetoftp.exe -f C:\Temp\foo\ -s ftp.example.com -p 21 -t /pub/dropbox/ -u joe -P mySecret -d

### Flags
    -d, --delete     Deletes emptied folders after moving files
    -h, --help       Prints help information
    -V, --version    Prints version information

### Options
    -f, --from <PATH>            The path to the local folder where the files will be moved from [default: .]
    -P, --password <PASSWORD>    The password of the user on the FTP-server to login with
    -p, --port <PORT>            The hostname of the FTP-server [default: 21]
    -s, --server <HOST>          The hostname of the FTP-server
    -t, --to <PATH>              The remote path on FTP-server where the files will be moved to
    -u, --username <USERNAME>    The username of the user on the FTP-server to login with
