/// 为字符串生成哈希字符串
pub fn hash(pwd: &str) -> Result<String, String> {
    bcrypt::hash(pwd, bcrypt::DEFAULT_COST).map_err(|err| err.to_string())
}

/// 验证密码是否匹配哈希字符串
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool, String> {
    bcrypt::verify(pwd, hashed_pwd).map_err(|err| err.to_string())
}

/// 密钥
const KEY: [i32; 16] = [1, 3, 5, 8, 1, 9, 0, 7, 4, 0, 6, 12, 13, 14, 15, 16];
/// nonce 随机数
const IV: [i32; 16] = [0x12; 16];

/// AES 加密
pub fn aes_encrypt(input: String) -> String {
    // 引入加密算法库
    use aes::cipher::{KeyIvInit, StreamCipher};

    // 定义类型别名
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

    // 字符串转字节数组
    let mut buf = input.into_bytes();

    // 加密消息
    let mut cipher = Aes128Ctr64LE::new(&KEY.into(), &IV.into());
    cipher.apply_keystream(&mut buf);

    // 返回密文
    const_hex::encode(&buf)
}

/// AES 解密
pub fn aes_decrypt(input: String) -> String {
    // 引入加密算法库
    use aes::cipher::{KeyIvInit, StreamCipher};

    // 定义类型别名
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

    // 十六进制字符串转字节数组
    let mut buf = const_hex::decode(input).unwrap();

    // 解密消息
    let mut cipher = Aes128Ctr64LE::new(&KEY.into(), &IV.into());
    cipher.apply_keystream(&mut buf);

    // 返回明文
    unsafe { String::from_utf8_unchecked(buf) }
}
