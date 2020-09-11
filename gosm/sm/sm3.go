package sm

import (
	"encoding/hex"
	"fmt"
	"github.com/tjfoc/gmsm/sm3"
)

func Sm3(data string)string {
	fmt.Println("sm3data==",data)
	h := sm3.New()
	h.Write([]byte(data))
	sum := h.Sum(nil)

	str := hex.EncodeToString(sum)
	return str
}
func Testsm3(){
	s:="adfaas;dfa;sldjf;asj"
	res:=Sm3(s)
	fmt.Println(res,len(res))
}
