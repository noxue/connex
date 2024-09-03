use connex::{
    core::{
        filter::{
            error::{FilterError, FilterResult},
            Filter,
        },
        pipeline::Pipeline,
    },
    server::connection::Connection,
};
use tokio::net::{TcpListener, TcpStream};

struct CompressionFilter;

impl Filter for CompressionFilter {
    fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>> {
        let mut compressed_data = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let current = data[i];
            let mut count = 1;

            while i + count < data.len() && data[i + count] == current {
                count += 1;
            }

            compressed_data.push(current);
            compressed_data.push(count as u8);
            i += count;
        }

        Ok(compressed_data)
    }
}

struct DecompressionFilter;

impl Filter for DecompressionFilter {
    fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>> {
        let mut decompressed_data = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let value = data[i];
            let count = if i + 1 < data.len() {
                data[i + 1]
            } else {
                return Err(FilterError::UserDefinedError {
                    message: "Invalid compressed data format".to_string(),
                });
            };

            decompressed_data.extend(vec![value; count as usize]);
            i += 2;
        }

        Ok(decompressed_data)
    }
}

struct EncryptionFilter;

impl Filter for EncryptionFilter {
    fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>> {
        let mask: u8 = 0xAA; // 可以选择其他掩码值
        let encrypted_data: Vec<u8> = data.iter().map(|&b| b ^ mask).collect();
        Ok(encrypted_data)
    }
}

struct DecryptionFilter;

impl Filter for DecryptionFilter {
    fn process(&self, data: Vec<u8>) -> FilterResult<Vec<u8>> {
        let mask: u8 = 0xAA;
        let decrypted_data: Vec<u8> = data.iter().map(|&b| b ^ mask).collect();
        Ok(decrypted_data)
    }
}






#[tokio::main]
async fn main() {


    // set global subscriber
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    

    let server_handle = tokio::spawn(async {
        start_server().await;
    });

    let client_handle = tokio::spawn(async {
        start_client().await;
    });

    let _ = tokio::join!(server_handle, client_handle);
}

async fn start_server() {
    let tcp_listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();

    loop {
        let (socket, _) = tcp_listener.accept().await.unwrap();
        tokio::spawn(async move {



            // you can impl exchange key, create encryption filter and decryption filter here

           

            
            let (reader, writer) = socket.into_split();
            let mut connection = Connection::new(reader, writer);

            // for example, I add compression filter
            connection.add_filter_in(Box::new(DecompressionFilter));
            connection.add_filter_out(Box::new(CompressionFilter));

            tracing::info!("Connection started");

            connection.start().await;
        });
    }
}

async fn start_client() {
    let tcp_stream = TcpStream::connect("127.0.0.1:12345").await.unwrap();


    // you can impl exchange key, create encryption filter and decryption filter here

    let (reader, writer) = tcp_stream.into_split();

    

    let mut connection = Connection::new(reader, writer);


    // for example, I add compression filter
    connection.add_filter_in(Box::new(DecompressionFilter));
    connection.add_filter_out(Box::new(CompressionFilter));

    tracing::info!("Client connected");

    connection.start().await;

}
