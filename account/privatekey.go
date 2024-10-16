package account

import "crypto/rand"

// NewSeed creates a uniformly random 32-byte account seed.
func NewSeed() ([64]byte, error) {

	var d [64]byte
	if _, err := rand.Read(d[:]); err != nil {
		return [64]byte{}, err
	}
	return d, nil
}
