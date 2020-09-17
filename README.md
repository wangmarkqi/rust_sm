# rust_sm rust的国密实现 

## 注意：
- win首次运行cargo run，调用动态链接(源代码目录下dll文件)，运行时build.rs自动将动态链接拷贝到OUT_DIR,但是如果cargo build产生可执行文件，运行可执行文件，但是运行时无法找到OUT_DIR位置，需要设置环境变量OUT_DIR，并且将libgosm.dll移动到这个文件夹。
- linux下运行cargo run 和build，由于采用静态库链接打包，没有任何影响。
- 所有代码在linux系统和win10经过测试,win只能使用gnu,不可以msvc。
- win10设置环境变量: set OUT_DIR=<你存放libgosm.dll文件的文件夹位置>。
- linux无需设置。
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