use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{mpsc, oneshot};
use std::rc::Rc;
use std::sync::Arc;
use tokio::{task, time};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use std::str;

/* async fn say_world() {
   println!("world");
   }

//#[tokio::main]
fn main(){

let mut rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {

let op  = say_world();

println!("hello");

op.await;
})
}*/

#[allow(dead_code)]
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get 
    { 
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set 
    { 
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    /* let v = Arc::new(vec![1,2,3]);
       let v1 = Arc::clone(&v);
       task::spawn(async move {
       println!("Here is a Vec: {:?}", v1);
       });

       let v2 = Arc::clone(&v);
       task::spawn(async move {
       println!("Here is a Another Vec: {:?}", v2);
       }); */

    /* tokio::spawn(async {

    //let rc = Arc::new("hello"); Compile Error Not Implement Send Trait
    let arc = Arc::new("hello");

    task::yield_now().await;

    println!("{}",arc);
    });

    time::sleep(time::Duration::from_secs(1));*/

    /*
    // Open a connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with the value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result = {:?}", result);*/


    /* let mut client = client::connect("127.0.0.1:6379").await.unwrap();
       let t1 = tokio::spawn(async {
       let res = client.get("hello").await;
       });

       let t2 = tokio::spawn(async {
       let res = client.set("foo", "bar".into()).await;
       });

       t1.await.unwrap();
       t2.await.unwrap();*/


    /* // Construct mpsc channel
       let (tx, mut rx) = mpsc::channel(32);

    // Clone a handle for tx
    let tx2  = tx.clone();

    // Manager task 
    // recv mpsc channel and process connection from server 
    // then send the process result back according to the mpsc recv 
    let manager = tokio::spawn(async move{
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Recv mpsc channel from multiple sender tasks
    while let Some(cmd) = rx.recv().await {
    match cmd {
    Command::Get{key, resp} =>{
    let res = client.get(&key).await;
    // Ignore errors
    let  _ = resp.send(res);
    }
    Command::Set{key, val, resp} =>{
    let res = client.set(&key, val).await;
    // Ignore errors
    let  _ = resp.send(res);
    }
    }
    }
    });

    // Spawn two tasks , one setting a value and other querying for key that was set
    let t1 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();

    // Construct the GET request param
    let cmd = Command::Get {
    key: "foo".to_string(),
    resp: resp_tx,
    };

    // Send the GET request
    if tx.send(cmd).await.is_err()
    {
    eprintln!("connection task shutdown");
    return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();

    // Construct the SET request param
    let cmd = Command::Set {
    key: "foo".to_string(),
    val: "bar".into(),
    resp: resp_tx,
    };

    // Send the SET request
    if tx2.send(cmd).await.is_err()
    {
    eprintln!("connection task shutdown");
    return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Set) = {:?}", res);
    });
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();*/

    // AsyncIO Operation
    let mut f= File::open("foo.txt").await?;
    let mut buffer = [0;100];
    let n = f.read(&mut buffer[..]).await?;
    println!("The total {:?} bytes: {:?}", n ,&buffer[..n]);
    let recv_str = str::from_utf8(&buffer[..n]);
    println!("The total {:?} str: {:?}", n, recv_str);

    let mut f= File::open("foo.txt").await?;
    let mut buffer2 = Vec::new();
    f.read_to_end(&mut buffer2).await?;
    println!("The total {:?} bytes: {:?}", n ,&buffer2[..n]);
    let recv_str = str::from_utf8(&buffer2[..n]);
    println!("The total {:?} str: {:?}", n, recv_str);

    let mut f = File::create("bar.txt").await?;
    let n = f.write(b"love rust by write single \n").await?;
    println!("The total write {:?} bytes", n);

    let mut f = File::create("bar2.txt").await?;
    let n = f.write_all(b"love rust by write all \n").await?;
    println!("The total write {:?} bytes", n);

    let mut reader: &[u8] = b"love rust third time by copy \n";
    let mut f = File::create("bar3.txt").await?;
    io::copy(&mut reader, &mut f).await?;

    Ok(())
}
