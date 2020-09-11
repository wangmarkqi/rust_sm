package sm

import (
	"encoding/hex"
	"fmt"
	"github.com/tjfoc/gmsm/sm2"
	"log"
	"math/big"
	"strings"
)
const key = "sm"
const split = "******************************"
func GenKeyPair()string {
	priv, _ := sm2.GenerateKey() // 生成密钥对
	pub := &priv.PublicKey
	pkok,err1:=sm2.WritePublicKeytoPem("pk.pem", pub, []byte(key))
	if err1!=nil{
		fmt.Println(pkok,err1)
		return "fail"
	}
	skok,err2:=sm2.WritePrivateKeytoPem("sk.pem", priv, []byte(key))
	if err2!=nil{
		fmt.Println(skok,err2)
		return "fail"
	}
	return "success"
}
func Sm2Enc(pkpath string, message string) string{
	fmt.Println("pkpath===",pkpath)
	fmt.Println("msg===",message)
	pub, err := sm2.ReadPublicKeyFromPem(pkpath, []byte(key))
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	msg:=[]byte(message)
	ciph, err := pub.Encrypt(msg)
	if err != nil {
		log.Fatal(err)
		return "fail"
	}
	str := hex.EncodeToString(ciph)
	return str
}
func Sm2Dec(skpath string, ciphertxt string) string {
	priv, err := sm2.ReadPrivateKeyFromPem(skpath, []byte(key))
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	b, err := hex.DecodeString(ciphertxt)
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	plaintxt, err := priv.Decrypt(b)
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}

	return string(plaintxt)
}

func Sm2Sign(skpath string, message string) string {
	priv, err := sm2.ReadPrivateKeyFromPem(skpath, []byte(key))
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	msg := []byte(message)
	r, s, err := sm2.Sign(priv, msg)
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	res := fmt.Sprintf("%x%s%x", r, split, s)
	return res

}
func Sm2Verify(pkpath string, message string, sign string) string {
	l := strings.Split(sign, split)

	rrr := new(big.Int)
	rrr.SetString(l[0], 16)
	sss := new(big.Int)
	sss.SetString(l[1], 16)

	pub, err := sm2.ReadPublicKeyFromPem(pkpath, []byte(key))
	if err!=nil{
		fmt.Println(err)
		return "fail"
	}
	msg := []byte(message)
	isok := sm2.Verify(pub, msg, rrr, sss)
	if isok{
		return "success"
	}
	return "fail"
}
func Testsm2(){
	//dir:="./"
	//GenKeyPair(dir)
	//msg:="adfasdfa"
	//enc:=Sm2Enc(pkpath,msg)
	//fmt.Println(enc)
	//dec:=Sm2Dec(skpath,enc)
	//fmt.Println(dec)
	//cer:=Sm2Sign(skpath,msg)
	//fmt.Println(cer)
	//out:=Sm2Verify(pkpath,msg,cer)
	//fmt.Println(out)

}

