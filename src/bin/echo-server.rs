use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop{
        let (mut socket, _) = listener.accept().await?;
        // Automatical io copying
        /*
         * 
         *         tokio::spawn(async move {
         *             let (mut rd, mut wr) = socket.split();
         *             if io::copy(&mut rd, &mut wr).await.is_err()
         *             {
         *                 eprintln!("failed to coopy");
         *             }
         *         });
         * 
         */

        // Manual copying
        tokio::spawn(async move {
            let mut buf = vec![0;1024];

            loop{
                match socket.read(&mut buf).await{
                    //  remote sender closed
                    Ok(0) => return, 

                    // normally recieved
                    Ok(n) =>
                    {
                        if socket.write_all(&buf[..n]).await.is_err()
                        {
                            return;
                        }
                    }

                    //unexpected socket error
                    Err(_) =>
                    {
                        return;
                    }
                }
            }

        });
    };

}
