#![allow(non_snake_case)]
#[allow(dead_code)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(unused_must_use)]
pub mod clib;

fn main() {
    test();
}
fn test() -> anyhow::Result<()> {
    clib::tools::Hi()?;
    let b = clib::sm2::GenKeyPair()?;
    dbg!(b);
    // let msg = "adsfads";
    // let a = clib::sm3::Sm3(msg)?;
    // dbg!(a);
    // let c = clib::sm2::Sm2Enc("pk.pem", "asdfa")?;
    // let e = clib::sm2::Sm2Dec("sk.pem", &c)?;
    // dbg!(e);
    // let f = clib::sm2::Sm2Sign("sk.pem", "asdfasdf")?;
    // let g = clib::sm2::Sm2Verify("pk.pem", "asdfasdf", &f)?;
    // dbg!(g);
    // let k = "1234567890abcdef";
    // let h = clib::sm4::Sm4Enc(k, "asdfa")?;
    // dbg!(&h);
    Ok(())
}
