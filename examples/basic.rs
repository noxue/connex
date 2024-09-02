use connex::core::{
    filter::{
        error::{FilterError, FilterResult},
        Filter,
    },
    pipeline::Pipeline,
};

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

fn main() {
    let mut encode_pipeline = Pipeline::new();
    encode_pipeline.add_filter(Box::new(EncryptionFilter));
    encode_pipeline.add_filter(Box::new(CompressionFilter));

    let mut decode_pipeline = Pipeline::new();
    decode_pipeline.add_filter(Box::new(DecompressionFilter));
    decode_pipeline.add_filter(Box::new(DecryptionFilter));

    let data = b"Hello, world!".to_vec();

    let processed_data = encode_pipeline.process(data.clone()).unwrap();
    println!("Encoded and compressed data: {:?}", processed_data);

    let decoded_data = decode_pipeline.process(processed_data).unwrap();

    assert!(data == decoded_data);
}
