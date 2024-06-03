#[cfg(test)]
mod test {
    use aes::Block;
    use aes::cipher::{BlockDecrypt, KeyInit, StreamCipher, StreamCipherSeek};
    use aes::cipher::KeyIvInit;
    use hex_literal::hex;

    use crate::auth::JwtSecret;

    const SECRET: &str = "不负信赖";
    const ISS: &str = "圣农集团";

    #[test]
    fn test_gen_token() {
        let jwt = JwtSecret::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.create_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.to_token(&claims).unwrap();
        println!("{:?}", &token);
    }

    #[test]
    fn test_get_claims() {
        let jwt = JwtSecret::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.create_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.to_token(&claims).unwrap();
        let claims = jwt.verify_and_get(token.as_str()).unwrap();
        println!("{}", claims);
    }

    #[test]
    fn test_pwd_hash_verify() {
        let pwd = "123456";
        let hashed = crate::auth::password::hash(pwd).unwrap();
        dbg!(format!("hashed: {}", hashed));
        let verified = crate::auth::password::verify(pwd, &hashed).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_pwd_aes() {
        use aes::cipher::{KeyIvInit, StreamCipher};
        use hex_literal::hex;

        type Aes128Ctr64LE = ctr::Ctr128LE<aes::Aes128>;

        let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iv = [0x0; 16];
        let plaintext = *b"0000";
        let ciphertext = hex!("ebc1b421");

        // encrypt in-place
        let mut buf = plaintext.to_vec();
        let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
        cipher.apply_keystream(&mut buf);
        dbg!(format!("buf: {}", vec_to_hex_string(buf.clone())));
        dbg!(format!("ciphertext: {}", vec_to_hex_string(ciphertext.to_vec())));
        assert_eq!(buf[..], ciphertext[..]);

        // CTR mode can be used with streaming messages
        let mut cipher = Aes128Ctr64LE::new(&key. into(), &iv. into());
        for chunk in buf. chunks_mut(3) {
            cipher. apply_keystream(chunk);
        }
        // dbg!(format!("buf: {}", vec_to_hex_string(buf.clone())));
        dbg!(format!("buf: {}", String::from_utf8(buf.clone()).unwrap()));
        dbg!(format!("plaintext: {}", String::from_utf8(Vec::from(plaintext)).unwrap()));
        assert_eq!(buf[..], plaintext[..]);

        cipher.seek(0u32);
    }

    // helper 函数，将 Vec<u8> 转换为 hex 字符串
    fn vec_to_hex_string(vec: Vec<u8>) -> String {
        vec.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
}
