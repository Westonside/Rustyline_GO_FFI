package main

import "C"

//needs to start with lib
//go build -buildmode=c-archive -o libgofile.a main.go use to create a static binary

//export testing
func testing(something string) *C.char {
	return C.CString("testing")
}

func main() {}
