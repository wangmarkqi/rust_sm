#![allow(non_snake_case)]

#[allow(dead_code)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(unused_must_use)]
#[cfg(target_os = "linux")]
pub mod cstatic;

#[cfg(target_os = "windows")]
pub mod dynamic;

/// 生成国密sm2密钥对，运行目录下私钥sk.pem,公钥pk.pem
/// # Examples
/// ```
/// GenKeyPair();
/// ```
#[cfg(target_os = "windows")]
pub fn GenKeyPair() -> anyhow::Result<String> {
    let b = dynamic::sm2::GenKeyPair();
    b
}

#[cfg(target_os = "linux")]
pub fn GenKeyPair() -> anyhow::Result<String> {
    let b = cstatic::tools::genKeyPair();
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

#[cfg(target_os = "windows")]
pub fn Sm2Enc(pk: &str, msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm2::Sm2Enc(pk, msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm2Enc(pk: &str, msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm2Enc(pk, msg);
    c
}
/// sm2 解密

#[cfg(target_os = "windows")]
pub fn Sm2Dec(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm2::Sm2Dec(sk, msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm2Dec(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm2Dec(sk, msg);
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
#[cfg(target_os = "windows")]
pub fn Sm2Sign(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm2::Sm2Sign(sk, msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm2Sign(sk: &str, msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm2Sign(sk, msg);
    c
}
/// sm2  验签
#[cfg(target_os = "windows")]
pub fn Sm2Verify(pk: &str, msg: &str, sign: &str) -> anyhow::Result<String> {
    let c = dynamic::sm2::Sm2Verify(pk, msg, sign);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm2Verify(pk: &str, msg: &str, sign: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm2Verify(pk, msg, sign);
    c
}
/// sm3 摘要加密
/// # Examples
/// ```
/// let enc=Sm3("testmsg");
/// ```
#[cfg(target_os = "windows")]
pub fn Sm3(msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm3::Sm3(msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm3(msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm3(msg);
    c
}
/// sm4 加密
/// # Examples
/// ```
/// let k = "1234567890abcdef";
/// let enc=Sm4Enc(k,"testmsg");
/// let dec=SmeDec(k,&enc);
/// ```
#[cfg(target_os = "windows")]
pub fn Sm4Enc(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm4::Sm4Enc(k, msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm4Enc(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm4Enc(k, msg);
    c
}
/// sm4 解密
#[cfg(target_os = "windows")]
pub fn Sm4Dec(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = dynamic::sm4::Sm4Dec(k, msg);
    c
}
#[cfg(target_os = "linux")]
pub fn Sm4Dec(k: &str, msg: &str) -> anyhow::Result<String> {
    let c = cstatic::tools::sm4Dec(k, msg);
    c
}
///Hi for test
#[cfg(target_os = "windows")]
pub fn Hi() -> anyhow::Result<()> {
    dynamic::tools::Hi();
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn Hi() -> anyhow::Result<()> {
    cstatic::tools::hi();
    Ok(())
}
