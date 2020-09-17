#![allow(non_snake_case)]

#[allow(dead_code)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(unused_must_use)]
#[cfg(target_os = "linux")]
pub mod cstatic;

#[cfg(target_os = "windows")]
pub mod dynamic;

fn main() {
    test();
}
#[cfg(target_os = "windows")]
fn test() -> anyhow::Result<()> {
    dynamic::tools::Hi();
    dynamic::sm2::GenKeyPair();
    // let b = dynamic::sm2::GenKeyPair()?;
    // dbg!(b);
    let msg = "adsfads";
    let a = dynamic::sm3::Sm3(msg)?;
    dbg!(a);
    let c = dynamic::sm2::Sm2Enc("pk.pem", "asdfa")?;
    let e = dynamic::sm2::Sm2Dec("sk.pem", &c)?;
    dbg!(e);
    let f = dynamic::sm2::Sm2Sign("sk.pem", "asdfasdf")?;
    let g = dynamic::sm2::Sm2Verify("pk.pem", "asdfasdf", &f)?;
    dbg!(g);
    let k = "1234567890abcdef";
    let h = dynamic::sm4::Sm4Enc(k, "asdfa")?;
    dbg!(&h);
    let i = dynamic::sm4::Sm4Dec(k, &h)?;
    dbg!(&i);
    Ok(())
}
#[cfg(target_os = "linux")]
fn test() -> anyhow::Result<()> {
    cstatic::tools::hi();
    let b = cstatic::tools::genKeyPair()?;
    dbg!(b);
    let msg = "adsfads";
    let a = cstatic::tools::sm3(msg)?;
    dbg!(a);
    let c = cstatic::tools::sm2Enc("pk.pem", "asdfa")?;
    let e = cstatic::tools::sm2Dec("sk.pem", &c)?;
    dbg!(e);
    let f = cstatic::tools::sm2Sign("sk.pem", "asdfasdf")?;
    let g = cstatic::tools::sm2Verify("pk.pem", "asdfasdf", &f)?;
    dbg!(g);
    let k = "1234567890abcdef";
    let h = cstatic::tools::sm4Enc(k, "asdfa")?;
    dbg!(&h);
    let i = cstatic::tools::sm4Dec(k, &h)?;
    dbg!(&i);
    Ok(())
}
