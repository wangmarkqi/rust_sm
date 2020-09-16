# rust_sm rust的国密实现 

## 注意：
- 首次运行cargo run，调用动态链接(源代码目录下so和dll文件)，运行时build.rs自动根据操作系统将动态链接 拷贝到OUT_DIR,
- 如果cargo build产生可执行文件，运行可执行文件，但是运行时无法找到OUT_DIR位置，需要设置环境变量OUT_DIR，并且将libgosm.so或者libgosm.dll移动到这个文件夹。
- 所有代码在linux系统和win10经过测试,win只能使用gnu,不可以msvc。
- win10设置环境变量: set OUT_DIR=C://mylib
- linux设置环境变量: export OUT_DIR="/home/mylib"
- 文档：https://docs.rs/rust_sm/0.1.4/rust_sm/

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