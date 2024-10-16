package account

/*
   #cgo LDFLAGS: -L${SRCDIR}/../aleo/target/release -laleo
   #include <stdlib.h>
   char* pk_str_from_str(const char* pk_str);
   char* vk_str_from_pk(const char* pk_str);
   char* add_str_from_pk(const char* pk_str);
   void free_rust_string(char* s);
   char* pk_from_seed(const uint8_t *n, size_t len);
*/
import "C"
import (
	"bytes"
	"errors"
	"unsafe"
)

var errInvalidSeed = errors.New("invalid seed")

func AleoAccountFromPrivateKey(privateKey string) (*Account, error) {
	// 将 Go 字符串转换为 C 字符串
	cPrivateKey := C.CString(privateKey)
	defer C.free(unsafe.Pointer(cPrivateKey))
	// 调用 Rust 函数
	cPk := C.pk_str_from_str(cPrivateKey)
	// 将结果转换回 Go 字符串
	goPk := C.GoString(cPk)
	// 打印结果
	// 释放 Rust 分配的内存
	C.free_rust_string(cPk)
	cVk := C.vk_str_from_pk(cPrivateKey)
	goVk := C.GoString(cVk)
	C.free_rust_string(cVk)
	cadd := C.add_str_from_pk(cPrivateKey)
	goAdd := C.GoString(cadd)
	C.free_rust_string(cadd)
	return &Account{
		privateKey: goPk,
		viewKey:    goVk,
		address:    goAdd,
	}, nil
}

func AleoAccountFromSeed(seedFromUser [64]byte) (*Account, error) {
	var seed [64]byte
	var err error
	if bytes.Equal(seedFromUser[:], make([]byte, 64)) {
		seed, err = NewSeed()
		if err != nil {
			return nil, err
		}
	} else {
		seed = seedFromUser
	}

	cPk := C.pk_from_seed((*C.uint8_t)(unsafe.Pointer(&seed[0])), C.size_t(len(seed)))
	if cPk == nil {
		return nil, nil
	}
	// 将结果转换回 Go 字符串
	goPk := C.GoString(cPk)
	// 打印结果
	// 释放 Rust 分配的内存
	C.free_rust_string(cPk)

	cPrivateKey := C.CString(goPk)
	vk := C.vk_str_from_pk(cPrivateKey)
	goVk := C.GoString(vk)
	C.free_rust_string(vk)
	add := C.add_str_from_pk(cPrivateKey)
	goAdd := C.GoString(add)
	C.free_rust_string(add)
	return &Account{
		privateKey: goPk,
		viewKey:    goVk,
		address:    goAdd,
	}, nil

}
