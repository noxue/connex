use crate::server::connection::Connection;



pub struct Server{
    port: u32,
    address: String,
}

impl Server {
    pub fn new(port: u32, address: String) -> Server {
        Server {
            port,
            address,
        }
    }

    pub async fn start(&self) {
        println!("Server started on {}:{}", self.address, self.port);
        // start tokio tcp server
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", self.address, self.port)).await.unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let (reader, writer) = socket.into_split();
                let mut connection = Connection::new(reader, writer);
                connection.start().await;
            });
        }
    }
}
