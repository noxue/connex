use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum FilterError {
    #[snafu(display("Failed to process data: {}", source))]
    ProcessError { source: std::io::Error },

    #[snafu(display("Encryption error: {}", source))]
    EncryptionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Decryption error: {}", source))]
    DecryptionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Compression error: {}", source))]
    CompressionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Decompression error: {}", source))]
    DecompressionError { source: Box<dyn std::error::Error> },

    #[snafu(display("Invalid input data: {}", message))]
    InvalidData { message: String },

    #[snafu(display("{}", message))]
    UserDefinedError { message: String },
}

pub type FilterResult<T> = std::result::Result<T, FilterError>;
