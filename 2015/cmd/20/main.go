package main

import (
	"fmt"
	"math"
)

const INPUT int = 29000000

func main() {

	p1 := 0
	gifts := 0
	for gifts < INPUT {
		p1 += 1
		houseGifts := 0
		for elf := range int(math.Sqrt(float64(p1))) {
			if p1%(elf+1) == 0 {
				houseGifts += 10 * (elf + 1)
				if p1/(elf+1) != (elf + 1) {
					houseGifts += 10 * (p1 / (elf + 1))
				}
			}
		}
		if houseGifts > gifts {
			gifts = houseGifts
		}
	}

	p2 := 0
	elves := map[int]int{}
	gifts = 0
	for gifts < INPUT {
		p2 += 1
		houseGifts := 0
		for elf := range int(math.Sqrt(float64(p1))) {
			if p2%(elf+1) == 0 {
				if v, ok := elves[elf+1]; ok {
					if v <= 50 {
						houseGifts += 11 * (elf + 1)
						elves[elf+1] += 1
					}
				} else {
					houseGifts += 11 * (elf + 1)
					elves[elf+1] += 1
				}
				if p2/(elf+1) != (elf + 1) {
					if v, ok := elves[p2/(elf+1)]; ok {
						if v <= 50 {
							houseGifts += 11 * (p2 / (elf + 1))
							elves[p2/(elf+1)] += 1
						}
					} else {
						houseGifts += 11 * (p2 / (elf + 1))
						elves[p2/(elf+1)] += 1
					}
				}
			}
		}
		if houseGifts > gifts {
			gifts = houseGifts
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
