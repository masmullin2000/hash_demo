use std::{fs::File, io::Read};

use anyhow::Result;
pub use sha2::{Digest, Sha256, Sha512};

const SIZE: usize = 0xFFFF;

pub fn hash_file<D: Digest>(file_name: &str) -> Result<Vec<u8>> {
    let mut hasher = D::new();
    let mut file = File::open(file_name)?;
    let mut file_data = vec![0; SIZE];

    loop {
        let amt_data_read = file.read(&mut file_data)?;
        if amt_data_read == SIZE {
            hasher.update(&file_data);
        } else {
            hasher.update(&file_data[0..amt_data_read]);
            break;
        }
    }

    let hash = hasher.finalize();
    let sz = <D as Digest>::output_size();
    let mut ret = vec![0; sz];

    ret.copy_from_slice(&hash);

    Ok(ret)
}

pub fn hex_to_string(data: &[u8]) -> String {
    let mut ret = String::new();

    for d in data {
        let x = format!("{:02x}", d);
        ret.push_str(&x);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_sum() {
        let hash = hash_file::<Sha256>("test_data/test1.bin").expect("didn't hash properly");
        let hash_string = hex_to_string(&hash);
        let expected_result = "3c593aa539fdcdae516cdf2f15000f6634185c88f505b39775fb9ab137a10aa2";
        assert_eq!(&hash_string, expected_result);
    }

    #[test]
    fn test_sha512_sum() {
        let hash = hash_file::<Sha512>("test_data/test_512.bin").expect("didn't hash properly");
        let hash_string = hex_to_string(&hash);
        let expected_result = "a9db490c708cc72548d78635aa7da79bb253f945d710e5cb677a474efc7c65a2aab45bc7ca1113c8ce0f3c32e1399de9c459535e8816521ab714b2a6cd200525";
        assert_eq!(&hash_string, expected_result);
    }
}
