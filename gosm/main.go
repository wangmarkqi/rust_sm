package main

import "C"
import (
	"fmt"
	"gosm/sm"
)

func main() {
}
//export Hi
func Hi() {
	fmt.Println("hello!!!!!!!!!!")
}
//export GenKeyPair
func GenKeyPair()*C.char {
	s:=sm.GenKeyPair()
	return C.CString(s)
}
//export Sm2Enc
func Sm2Enc(pkpath *C.char, message *C.char) *C.char {
	pk:=C.GoString(pkpath)
	me:=C.GoString(message)
	s:=sm.Sm2Enc(pk, me)
	return C.CString(s)
}

//export Sm2Dec
func Sm2Dec(skpath *C.char, cipher *C.char) *C.char {
	sk:=C.GoString(skpath)
	ci:=C.GoString(cipher)
	s:=sm.Sm2Dec(sk, ci)
	return C.CString(s)
}

//export Sm2Sign
func Sm2Sign(skpath *C.char, message *C.char) *C.char {
	sk:=C.GoString(skpath)
	me:=C.GoString(message)
	s:=sm.Sm2Sign(sk, me)
	return C.CString(s)
}

//export Sm2Verify
func Sm2Verify(pkpath *C.char, message *C.char, sign *C.char) *C.char{
	pk:=C.GoString(pkpath)
	me:=C.GoString(message)
	si:=C.GoString(sign)
	s:=sm.Sm2Verify(pk, me, si)
	return C.CString(s)
}

//export Sm3
func Sm3(d *C.char) *C.char {
	data:=C.GoString(d)
	s:=sm.Sm3(data)
	return C.CString(s)
}

//export Sm4Enc
func Sm4Enc(key *C.char, data *C.char)  *C.char{
	k:=C.GoString(key)
	d:=C.GoString(data)
	s:=sm.Sm4Enc(k, d)
	return C.CString(s)
}

//export Sm4Dec
func Sm4Dec(key *C.char, data *C.char) *C.char{
	k:=C.GoString(key)
	d:=C.GoString(data)
	s:=sm.Sm4Dec(k, d)
	return C.CString(s)
}



