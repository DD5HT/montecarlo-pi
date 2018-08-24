package main

import (
	"math"
	"math/rand"
	"time"
)

const iterations = 100000000

func inCircle(x, y float64) bool {
	return math.Sqrt(x*x+y*y) <= 1.0
}

func main() {
	source := rand.NewSource(time.Now().Unix())
	r := rand.New(source)
	var h int
	for i := 0; i <= iterations; i++ {
		if inCircle(r.Float64(), r.Float64()) {
			h++
		}
	}
	pi := 4 * float64(h) / float64(iterations)
	println("Pi ~ ", pi)
}
