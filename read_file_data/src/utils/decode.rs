use std::io;

/// Get the decoded data
///
/// # Arguments
///
/// * `data` - This is the which needs to be decoded
///
/// # Return
///
/// This function returns decoded(de-packet) data
pub fn decode<R: io::Read>(mut data: R) -> Vec<u8>{
    let mut output = vec![];
    loop {
        let mut len = 0u8;
        match data.read_exact(std::slice::from_mut(&mut len)) {
            Ok(_) => {
                let len = 1 << (len - 1);
                let mut buf = vec![0; len];
                let _res = data.read_exact(&mut buf);
                if buf == b"\0" {
                    break;
                }
                output.extend(buf);
            },
            Err(_) => break
        }
    }
    output
}

#[cfg(test)]
mod test {
    use crate::utils::decode::decode;

    #[test]
    fn test_decode_success() {
        let data = "{ x: 579, y: -197 , z: -485 }\0";
        let decoded = decode(data.as_bytes());
        assert_eq!(std::str::from_utf8(&decoded).unwrap(), "{ x: 579, y: -197 , z: -485 }");
    }

}

