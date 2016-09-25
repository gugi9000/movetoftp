# Move to FTP

A tool to upload local files to FTP and **delete them**, locally. Hence move to FTP :smile:


### Usage:

move-to-ftp.exe -f C:\Temp\foo\* -s ftp.example.com -p 21 -t /pub/dropbox/ -u joe -P mySecret -d

**When using -d foo in C:\Temp will be empty and will be deleted !**

### Options:
    -f, --from PATH     set the path to the local folder where the files will
                        be moved from (default is current working directory)
    -s, --server HOST   set the hostname of the FTP-server (required)
    -p, --port PORT     set the port on the FTP-server (default: 21)
    -t, --to PATH       set the remote path on the FTP-server where the files
                        will be moved to
    -u, --username USERNAME
                        set the username of the user on the FTP-server to
                        login with (required)
    -P, --password PASSWORD
                        set the password of the user on the FTP-server to
                        login with (required)
    -d, --delete        deletes emptied folders after moving files
    -h, --help          prints this help
