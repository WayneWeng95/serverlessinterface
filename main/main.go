package main

import (
	"math/rand"

	"../serverlessinterface/reap"
)

func main() {
	for i := 0; i < 5; i++ {
		println(rand.Intn(10))
	}

	reap.Setup()

}
