use md5::{Digest, Md5};

pub fn check_chunk_md5(md5: &String, chunk: &Vec<u8>) -> bool {
    let mut hasher = Md5::new();
    hasher.update(chunk);
    let result = hasher.finalize();

    md5.as_bytes() == &result.to_vec()
}
