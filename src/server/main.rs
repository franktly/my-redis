use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[allow(dead_code)]

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

struct CanIncrement{
    mutex: Mutex<i32>,
}

impl CanIncrement{
	fn increment(&self) {
		let mut lock = self.mutex.lock().unwrap();
		*lock +=1;
	}
}

#[tokio::main]
async fn main()
{
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop{
        let (socket, _) = listener.accept().await.unwrap();

	let db = db.clone();

	println!("Accepted");
        // process(socket).await; single client request
        tokio::spawn(async move{
            //process_v1(socket).await;
            //process_v2(socket).await;
            process_v3(socket, db).await;
        });
    }
}

async fn process_v3(socket: TcpStream, db: Db)
{
    use mini_redis::Command::{self, Get, Set };

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap()
	{
		Set(cmd) =>{
			let mut db = db.lock().unwrap();
			db.insert(cmd.key().to_string(), cmd.value().clone());
			Frame::Simple("OK".to_string())
		}

		Get(cmd) =>{
			let db = db.lock().unwrap();
			if let Some(value) = db.get(cmd.key()){
				Frame::Bulk(value.clone().into())
			}else{
				Frame::Null
			}
		}

		cmd => panic!("unimplementated {:?}", cmd),
	};

        connection.write_frame(&response).await.unwrap();
    }
}

async fn process_v2(socket: TcpStream)
{
    use mini_redis::Command::{self, Get, Set };

    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap()
        {
            Set(cmd) =>{
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }

            Get(cmd) =>{
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }else{
                    Frame::Null
                }
            }

            cmd => panic!("unimplementated {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

async fn process_v1(socket: TcpStream)
{
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap()
    {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());

        connection.write_frame(&response).await.unwrap();
    }
}


