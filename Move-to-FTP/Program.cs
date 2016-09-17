using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;
using WinSCP;

namespace Move_to_FTP
{
    class Program
    {
        static int Main(string[] args)
        {
            Console.WriteLine("Move to FTP v.0.1");
            Console.WriteLine("=================");

            if (args.Length != 6)
            {
                Console.WriteLine("Usage: Move-to-FTP.exe <local path> <ftpserver> <port> <remote path> <username> <password>.");
                Console.WriteLine(" ");
                Console.WriteLine("Example: Move-to-FTP.exe c:\\Temp\\* ftp.example.com 21 /upload/ joe secret");
                Console.WriteLine(" ");
                Console.WriteLine("Files from local path will be recursively uploaded to FTP-server in remote path and then deleted locally");
                return 1;
            } else
            {
                Console.WriteLine("Moving {0} to {1}:{2}{3}", args);
                Console.WriteLine("");
                
            }
            try
            {
                string localPath = args[0];
                string remotePath = args[3];
                string ftpHostName = args[1];
                int ftpPortNumber = Int32.Parse(args[2]);
                string ftpUsername = args[4];
                string ftpPassword = args[5];

                Console.WriteLine("Connecting..");
                SessionOptions sessionOptions = new SessionOptions
                {
                    Protocol = Protocol.Ftp,
                    HostName = ftpHostName,
                    PortNumber = ftpPortNumber,
                    UserName = ftpUsername,
                    Password = ftpPassword,
                };

                using (Session session = new Session())
                {
                    // Connect
                    session.Open(sessionOptions);

                    // Upload files
                    TransferOptions transferOptions = new TransferOptions();
                    transferOptions.TransferMode = TransferMode.Binary;

                    TransferOperationResult transferResult;
                    transferResult = session.PutFiles(@localPath, remotePath, false, transferOptions);

                    // Throw on any error
                    transferResult.Check();

                    // Print results
                    foreach (TransferEventArgs transfer in transferResult.Transfers)
                    {
                        Console.WriteLine("Upload of {0} succeeded, deleting..", transfer.FileName);
                        // TODO: Option to delete folders left empty.
                        try
                        {
                            File.Delete(transfer.FileName);
                        }
                        catch (IOException deleteError)
                        {
                            Console.WriteLine(deleteError.Message);
                        }
                    }

                }
                return 0;
            }
            catch (Exception e)
            {
                Console.WriteLine("Error: {0}", e);
                //System.Threading.Thread.Sleep(5000); // Sleep X miliseconds before resuming.
                return 1;
            }
            
        } // Main
    } // class Program
} // namespace
