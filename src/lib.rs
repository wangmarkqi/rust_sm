#![allow(non_snake_case)]
#[allow(dead_code)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(unused_must_use)]
pub mod clib;

/// 生成国密sm2密钥对，运行目录下私钥sk.pem,公钥pk.pem
/// # Examples
/// ```
/// GenKeyPair();
/// ```
pub fn GenKeyPair() -> anyhow::Result<String> {
    let b = clib::sm2::GenKeyPair();
    b
}
/// sm2 加密
/// # Examples
/// ```
/// let pk="./pk.pem";
/// let sk="./sk.pem";
/// let enc=Sm2Enc(pk,"testmsg");
/// let dec=Sm2Dec(sk,&enc);
/// ```
pub fn Sm2Enc(pk: &str, msg: &str) -> anyhow::Result<String> {
    let c = clib::sm2::Sm2Enc(pk, msg);
    c
}
/// sm2 解密
pub fn Sm2Dec(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = clib::sm2::Sm2Dec(sk, msg);
    c
}
/// sm2 签名
/// # Examples
/// ```
/// let pk="./pk.pem";
/// let sk="./sk.pem";
/// let sign=Sm2Sign(sk,"testmsg");
/// let istrue=Sm2Verify(pk,"testmsg",&sign);
/// ```
pub fn Sm2Sign(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = clib::sm2::Sm2Sign(sk, msg);
    c
}
/// sm2  验签
pub fn Sm2Verify(pk: &str, msg: &str, sign: &str) -> anyhow::Result<String> {
    let c = clib::sm2::Sm2Verify(pk, msg, sign);
    c
}
/// sm3 摘要加密
/// # Examples
/// ```
/// let enc=Sm3("testmsg");
/// ```
pub fn Sm3(msg: &str) -> anyhow::Result<String> {
    let c = clib::sm3::Sm3(msg);
    c
}
/// sm4 加密
/// # Examples
/// ```
/// let k = "1234567890abcdef";
/// let enc=Sm4Enc(k,"testmsg");
/// let dec=SmeDec(k,&enc);
/// ```
pub fn Sm4Enc(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = clib::sm4::Sm4Enc(k, msg);
    c
}
/// sm4 解密
pub fn Sm4Dec(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = clib::sm4::Sm4Dec(k, msg);
    c
}
