package sm

import (
	"bytes"
	"crypto/cipher"
	"encoding/hex"
	"fmt"
	"github.com/tjfoc/gmsm/sm4"
	"log"
)
// 128比特密钥
//key := []byte("1234567890abcdef")
func Sm4Enc(k string,data string)string{
	// 128比特iv
	key:=[]byte(k)
	if len(key)!=16{
		panic("key的长度必须是16字节，请指定16个数字或者英文字符当密码")
	}
	iv := make([]byte, sm4.BlockSize)
	ciph,err := Sm4Encrypt(key,iv, []byte(data))
	if err != nil{
		log.Fatal(err)
		return "fail"
	}
	//加密结果hex str
	str := hex.EncodeToString(ciph)
	return str
}

func Sm4Dec(k string,ciph string)string{
	key:=[]byte(k)
	if len(key)!=16{
		panic("key的长度必须是16字节，请指定16个数字或者英文字符当密码")
	}
	//hex str--->[]byte
	data, _ := hex.DecodeString(ciph)
	iv := make([]byte, sm4.BlockSize)
	dec,err := Sm4Decrypt(key,iv, data)
	if err != nil{
		log.Fatal(err)
		return "fail"
	}
	return string(dec)
}
func Sm4Encrypt(key, iv, plainText []byte) ([]byte, error) {
	block, err := sm4.NewCipher(key)
	if err != nil {
		return nil, err
	}
	blockSize := block.BlockSize()
	origData := Pkcs5Padding(plainText, blockSize)
	blockMode := cipher.NewCBCEncrypter(block, iv)
	cryted := make([]byte, len(origData))
	blockMode.CryptBlocks(cryted, origData)
	return cryted, nil
}

func Sm4Decrypt(key, iv, cipherText []byte) ([]byte, error) {
	block, err := sm4.NewCipher(key)
	if err != nil {
		return nil, err
	}
	blockMode := cipher.NewCBCDecrypter(block, iv)
	origData := make([]byte, len(cipherText))
	blockMode.CryptBlocks(origData, cipherText)
	origData = Pkcs5UnPadding(origData)
	return origData, nil
}
// pkcs5填充
func Pkcs5Padding(src []byte, blockSize int) []byte {
	padding := blockSize - len(src)%blockSize
	padtext := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(src, padtext...)
}

func Pkcs5UnPadding(src []byte) []byte {
	length := len(src)
	if(length==0){
		return nil
	}
	unpadding := int(src[length-1])
	return src[:(length - unpadding)]
}

func Testsm4(){
	// 128比特密钥
	key := "1234567890abcdef"
	// 128比特iv
	data := "asdfa"
	ciphertxt := Sm4Enc(key, data)
	fmt.Println(ciphertxt)
	dec:=Sm4Dec(key,ciphertxt)
	fmt.Println(dec)


}