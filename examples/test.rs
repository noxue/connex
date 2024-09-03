use tokio::net::{TcpListener, TcpStream};

struct Writer;
struct Reader;
struct StopHandle;

impl StopHandle {
    async fn stop(&self) {
        // stop connection
    }
}

trait Filter {
    fn process(&self, data: Vec<u8>) -> Vec<u8>;
}

trait KeyExchange {
    fn process(&self, public_key: &str, private_key: &str) -> String;
}


#[derive(Clone)]
struct Connex;
impl Connex {
    pub fn new() -> Self {
        Connex
    }

    pub fn add_algorithm(
        &mut self,
        key_exchange: Box<dyn KeyExchange>,
        encryption_filter: Box<dyn Filter>,
        decryption_filter: Box<dyn Filter>,
    ) {
        // add key exchange, encryption filter and decryption filter
    }

    pub fn add_filter_in(&mut self, filter: Box<dyn Filter>) {
        // add filter to pipeline_in
    }

    pub fn add_filter_out(&mut self, filter: Box<dyn Filter>) {
        // add filter to pipeline_out
    }

    pub async fn accept(&self, socket: TcpStream) -> Connection {
        // attach connection
        Connection {
            session_id: "session_id".to_string(),
            writer: Writer,
            reader: Reader,
            stop_handle: StopHandle,
        }
    }
}

pub struct Connection {
    pub session_id: String,
    pub writer: Writer,
    pub reader: Reader,
    pub stop_handle: StopHandle,
}

impl Connection {
    pub async fn stop(&self) {
        self.stop_handle.stop().await;
    }
}

struct DemoEncryptionFilter;

impl DemoEncryptionFilter {
    pub fn new() -> Self {
        DemoEncryptionFilter
    }
}

impl Filter for DemoEncryptionFilter {
    fn process(&self, data: Vec<u8>) -> Vec<u8> {
        // do encryption
        data
    }
}

struct DemoDecryptionFilter;

impl DemoDecryptionFilter {
    pub fn new() -> Self {
        DemoDecryptionFilter
    }
}

impl Filter for DemoDecryptionFilter {
    fn process(&self, data: Vec<u8>) -> Vec<u8> {
        // do decryption
        data
    }
}

struct DemoCompressionFilter;

impl DemoCompressionFilter {
    pub fn new() -> Self {
        DemoCompressionFilter
    }
}

impl Filter for DemoCompressionFilter {
    fn process(&self, data: Vec<u8>) -> Vec<u8> {
        // do compression
        data
    }
}

struct DemoDecompressionFilter;

impl DemoDecompressionFilter {
    pub fn new() -> Self {
        DemoDecompressionFilter
    }
}

impl Filter for DemoDecompressionFilter {
    fn process(&self, data: Vec<u8>) -> Vec<u8> {
        // do decompression
        data
    }
}

struct DemoKeyExchange;

impl DemoKeyExchange {
    pub fn new(public_key: &str, private_key: &str) -> Self {
        DemoKeyExchange
    }
}

impl KeyExchange for DemoKeyExchange {
    fn process(&self, public_key: &str, private_key: &str) -> String {
        // do key exchange
        "secret key".to_string()
    }
}

#[tokio::main]
async fn main() {
    let mut connex = Connex::new();

    // add key exchange, you can set public key and private key if needed
    let key_exchange = DemoKeyExchange::new("public_key", "private_key");
    // add algorithm
    connex.add_algorithm(
        // key exchange
        Box::new(key_exchange),
        // encryption filter
        Box::new(DemoEncryptionFilter::new()),
        // decryption filter
        Box::new(DemoDecryptionFilter::new()),
    );

    // add filter
    connex.add_filter_in(Box::new(DemoCompressionFilter::new()));
    connex.add_filter_out(Box::new(DemoDecompressionFilter::new()));

    // add other filter etc.

    // bind and start tcp server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (socket, _) = tcp_listener.accept().await.unwrap();
        let connex_clone = connex.clone();
        tokio::spawn(async move {
            // in accept function, it exchange key, negotiation sessionid,  reconnect if needed,
            // automatically handle network sub-package and sticky packets,(自动处理网络分包和粘包)
            // finally return reader and writer for read and write data from/to encrypted connection
            let conn: Connection = connex_clone.accept(socket).await;
            // do something with writer and reader
            

            // stop connection, if you want to stop connection
            conn.stop().await;
        });
    }
}
