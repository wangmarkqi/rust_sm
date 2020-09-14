# rust_sm rust的国密实现 

## 注意：调用动态链接(源代码目录下so和dll文件)，仅在linux系统经过测试。运行时根据操作系统将动态链接 拷贝到OUT_DIR。

## 生成国密sm2密钥对，运行目录下私钥sk.pem,公钥pk.pem
 ```
 GenKeyPair();
 ```

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