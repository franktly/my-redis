use std::sync::{Arc};
use std::rc::Rc;
use tokio::{task, time};
use tokio::sync::mpsc;
use mini_redis::{client, Result};
use bytes::Bytes;

/*
   async fn say_world() {
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

#[derive(Debug)]
enum Command{
    Get{
key: String,
    },
    Set{
key:String,
    val: Bytes,
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    /*
       let v = Arc::new(vec![1,2,3]);
       let v1 = Arc::clone(&v);
       task::spawn(async move {
       println!("Here is a Vec: {:?}", v1);
       });

       let v2 = Arc::clone(&v);
       task::spawn(async move {
       println!("Here is a Another Vec: {:?}", v2);
       });*/

    /*
       tokio::spawn(async {

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
let (tx)

let mut client = client::connect("127.0.0.1:6379").await.unwrap();
let t1 = tokio::spawn(async {
        let res = client.get("hello").await;
        });

let t2 = tokio::spawn(async {
        let res = client.set("foo", "bar".into()).await;
        });

t1.await.unwrap();
t2.await.unwrap();
Ok(())
    }
