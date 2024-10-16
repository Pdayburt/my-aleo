package bip39

import (
	"fmt"
	"my-aleo/account"
	"testing"
)

func TestFromEntroy(t *testing.T) {

	entropy, err := NewEntropy(128)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Println(len(entropy))
	mnemonic, err := NewMnemonic(entropy)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Println(mnemonic)
	seed := NewSeed(mnemonic, "")

	fmt.Println("len(seed)*8", len(seed)*8)

}
func TestFromEntropy(t *testing.T) {

	entropy, err := NewEntropy(192)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("entropy: %#v\n", entropy)
	mnemonic, err := NewMnemonic(entropy)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("mnemonic:%#v\n", mnemonic)
	seed := NewSeed(mnemonic, "")
	fmt.Printf("seed:%#v\n", seed)
	aleoAccount, err := account.AleoAccountFromSeed([64]byte(seed))
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("account:%#v\n", aleoAccount)

}
