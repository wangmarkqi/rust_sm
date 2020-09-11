# rust_sm rust的国密实现 

##注意：调用动态链接，仅在linux系统经过测试 

生成国密sm2密钥对，运行目录下私钥sk.pem,公钥pk.pem
pub fn GenKeyPair() -> anyhow::Result<String> {
   let b = clib::sm2::GenKeyPair();
    b
}
## sm2 加密 解密
 Examples
```
 let pk="./pk.pem";
 let sk="./sk.pem";
 let enc=Sm2Enc(pk,"testmsg");
 let dec=Sm2Dec(sk,&enc);
 ```
## sm2 签名验签
 Examples
```
let pk="./pk.pem";
 let sk="./sk.pem";
 let sign=Sm2Sign(sk,"testmsg");
 let istrue=Sm2Verify(pk,"testmsg",&sign);
 ```
  
## sm3 摘要加密
 Examples
 ```
 let enc=Sm3("testmsg");
 ```

## sm4 加密解密
 Examples
 ```
 let k = "1234567890abcdef";
 let enc=Sm4Enc(k,"testmsg");
 let dec=SmeDec(k,&enc);
 ```