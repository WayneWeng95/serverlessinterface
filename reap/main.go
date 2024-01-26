package main

import (
	"math/rand"
)

func main() {
	for i := 0; i < 5; i++ {
		println(rand.Intn(10))
	}
}
