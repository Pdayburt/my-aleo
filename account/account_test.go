package account

import (
	"fmt"
	"testing"
)

func TestAccount(t *testing.T) {

	pk := "APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25"
	account, err := AleoAccountFromPrivateKey(pk)
	if err != nil {

	}

	fmt.Printf("%#v\n", account)

	/*fmt.Println("===========================================")

	account1, err := AleoAccountFromSeed([64]byte{})
	if err != nil {
		t.Fatal(err)
	}

	fmt.Printf("%#v", account1)*/

	/*
		fmt.Println("===========================================")
		seed, err := NewSeed()
		if err != nil {
			t.Fatal(err)
		}
		fmt.Printf("%#v\n", seed)
		fmt.Printf("%#v\n", len(seed)*8)
		fmt.Println("===========================================")
		mnemonic, err := bip39.NewMnemonic(seed[:])
		if err != nil {
			t.Fatal(err)
		}
		fmt.Println("mnemonic : ", mnemonic)*/
}

func TestUtil(t *testing.T) {

}
