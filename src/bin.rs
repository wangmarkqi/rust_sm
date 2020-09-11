pub mod clib;

fn main() {
    test();
}
fn test() {
    clib::tools::Hi();
    let b = clib::sm2::GenKeyPair();
    dbg!(b);
    let msg = "adsfads";
    let a = clib::sm3::Sm3(msg);
    dbg!(a);
    let c = clib::sm2::Sm2Enc("pk.pem", "asdfa");
    let e = clib::sm2::Sm2Dec("sk.pem", &c.unwrap());
    let f = clib::sm2::Sm2Sign("sk.pem", "asdfasdf");
    let g = clib::sm2::Sm2Verify("pk.pem", "asdfasdf", &f.unwrap());
    dbg!(g);
    let k = "1234567890abcdef";
    let h = clib::sm4::Sm4Enc(k, "asdfa");
    let enc = h.unwrap();
    dbg!(&enc);
    let i = clib::sm4::Sm4Dec(k, &enc);
    dbg!(i);
}
