package account

import (
	"fmt"
	"testing"
)

func TestSeed(t *testing.T) {
	seed, err := NewSeed()
	if err != nil {
		t.Fatal(err)
	}
	for _, v := range seed {
		fmt.Printf("%#v", rune(v))
	}
	fmt.Println()
	fmt.Println(len(seed))
}
