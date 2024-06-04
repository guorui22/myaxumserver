#[cfg(test)]
mod test {
    use crate::auth::JwtSecret;
    use crate::auth::password::*;

    const SECRET: &str = "不负信赖";
    const ISS: &str = "圣农集团";

    /// 测试生成 token
    #[test]
    fn test_gen_token() {
        let jwt = JwtSecret::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.create_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.to_token(&claims).unwrap();
        dbg!(token);
    }

    /// 测试解析 token 信息为 claims
    #[test]
    fn test_get_claims() {
        let jwt = JwtSecret::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.create_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.to_token(&claims).unwrap();
        let claims = jwt.verify_and_get(token.as_str()).unwrap();
        dbg!(claims);
    }

    /// 测试密码哈希和验证
    #[test]
    fn test_pwd_hash_verify() {
        let pwd = "123456";
        let hashed = hash(pwd).unwrap();
        dbg!(format!("hashed: {}", hashed));
        let verified = verify(pwd, &hashed).unwrap();
        assert!(verified);
    }

    /// 测试 AES 加密和解密步骤
    #[test]
    fn test_pwd_aes() {
        use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

        // 类型别名 Aes128Ctr64LE 定义
        type Aes128Ctr64LE = ctr::Ctr128LE<aes::Aes128>;

        // 密钥、IV、明文、密文
        let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iv = [0x0; 16];
        let plaintext = *b"123456";
        let ciphertext:[u8;6] = const_hex::decode_to_array(b"eac3b7251b8f").unwrap();

        // 加密消息
        let mut buf = plaintext.to_vec();
        let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
        cipher.apply_keystream(&mut buf);
        dbg!(format!("buf: {}", const_hex::encode(buf.clone())));
        dbg!(format!("ciphertext: {}", const_hex::encode(ciphertext.to_vec())));
        assert_eq!(buf[..], ciphertext[..]);

        // CTR 模式可用于流式消息解密
        let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
        for chunk in buf.chunks_mut(5) {
            cipher.apply_keystream(chunk);
        }
        dbg!(format!("buf: {}", String::from_utf8_lossy(&buf)));
        dbg!(format!("plaintext: {}", String::from_utf8_lossy(&plaintext)));
        assert_eq!(buf[..], plaintext[..]);

        // CTR 模式支持加密计数器重置从 0 开始
        cipher.seek(0u32);
        // 把对一个缓冲区信息“加密”的结果保存到到另一个缓冲区
        // 输出缓冲区长度必须等于输入缓冲区长度
        let mut buf1 = vec![0u8; plaintext.len()].into_boxed_slice();
        cipher.apply_keystream_b2b(&plaintext, &mut buf1).unwrap();
        dbg!(format!("buf1: {}", const_hex::encode(buf1.to_vec())));
        dbg!(format!("ciphertext: {}", const_hex::encode(ciphertext.to_vec())));
        assert_eq!(buf1[..], ciphertext[..]);

        // CTR 模式支持加密计数器重置从 0 开始
        cipher.seek(0u32);
        // 把对一个缓冲区信息“解密”的结果保存到到另一个缓冲区
        // 输出缓冲区长度必须等于输入缓冲区长度
        let mut buf2 = vec![0u8; plaintext.len()].into_boxed_slice();
        cipher.apply_keystream_b2b(&buf1, &mut buf2).unwrap();
        dbg!(format!("buf2: {}", String::from_utf8_lossy(&buf2)));
        dbg!(format!("plaintext: {}", String::from_utf8_lossy(&plaintext)));
        assert_eq!(buf2[..], plaintext[..]);
    }

    /// 测试 AES 加密和解密函数
    #[test]
    fn test_aes_encrypt_decrypt() {
        let input = "123456";
        let ciphertext = aes_encrypt(input.to_string());
        dbg!(format!("ciphertext: {}", ciphertext));
        let plaintext = aes_decrypt(ciphertext);
        dbg!(format!("plaintext: {}", plaintext));
        assert_eq!(input, plaintext);
    }

}
