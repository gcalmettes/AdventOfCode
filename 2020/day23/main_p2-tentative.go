package main

import "fmt"

func main() {
	// current value -> next value
	cups := make(map[int]int)

	values := []int{3, 8, 9, 1, 2, 5, 4, 6, 7}
	l := len(values)

	_, max := minMaxSlice(values)
	next := max + 1

	for i, v := range values {
		if i != l-1 {
			cups[v] = values[i+1]
		} else {
			// cups[v] = values[0]
			cups[v] = next
		}
	}

	// for i, v := range values {
	// 	if i != l-1 {
	// 		cups[v] = values[i+1]
	// 	} else {
	// 		cups[v] = values[0]
	// 	}
	// }

	// fills in with one million numbers
	for len(cups) < 1000000-1 {
		cups[next] = next + 1
		if max < next+1 {
			max = next + 1
		}
		next++

	}
	cups[next] = values[0]
	// fmt.Println(max, len(cups), cups[next])

	// nRounds := 100
	nRounds := 10000000
	i := 0
	currentCup := values[0]

	// order := []int{}
	// seen := make(map[int]bool)
	// c := currentCup
	// for !seen[c] {
	// 	seen[c] = true
	// 	order = append(order, c)
	// 	c = cups[c]
	// }
	// fmt.Println(order)

	for i < nRounds {
		if i%100000 == 0 {
			fmt.Println(i)
		}
		// remove the nextThree from the loop
		nextThreeStart := cups[currentCup]
		nextThreeMiddle := cups[nextThreeStart]
		nextThreeEnd := cups[nextThreeMiddle]
		// fmt.Println("  -- cut", nextThreeStart, nextThreeMiddle, nextThreeEnd)

		cups[currentCup] = cups[nextThreeEnd]

		// get min and max of left over cups
		// min, max := minMax(&cups, currentCup, nextThreeStart, nextThreeEnd)

		// destination
		var ok bool
		destinationCup := currentCup - 1
		for {
			if destinationCup == nextThreeStart || destinationCup == nextThreeMiddle || destinationCup == nextThreeEnd {
				destinationCup--
				continue
			}
			_, ok = cups[destinationCup]
			if !ok {
				destinationCup--
			} else {
				break
			}
			if destinationCup == 0 {
				destinationCup = max
				break
			}

		}

		// reinsert the three
		toLink := cups[destinationCup]
		cups[destinationCup] = nextThreeStart
		cups[nextThreeEnd] = toLink

		currentCup = cups[currentCup]

		// order := []int{}
		// seen := make(map[int]bool)

		// c := currentCup
		// for !seen[c] {
		// 	seen[c] = true
		// 	order = append(order, c)
		// 	c = cups[c]
		// }
		// fmt.Println(order)
		i++
	}

	a := cups[1]
	b := cups[a]
	fmt.Println(a * b)
}

type node struct {
	value, next int
}

func find(slice []int, value int) (int, bool) {
	for i, item := range slice {
		if item == value {
			return i, true
		}
	}
	return -1, false
}

func minMaxSlice(slice []int) (int, int) {
	var max int = slice[0]
	var min int = slice[0]
	for _, value := range slice {
		if max < value {
			max = value
		}
		if min > value {
			min = value
		}
	}
	return min, max
}

// func minMax(m *map[int]int, start, avoidStart, avoidEnd int) (int, int) {
// 	max := m[start]
// 	min := m[start]
// 	current := m[start]

// 	skip := false
// 	for current != start {
// 		value := m[current]
// 		if value == avoidStart {
// 			skip = true
// 		}
// 		if value == m[avoidEnd] {
// 			skip = false
// 		}
// 		if !skip {
// 			if max < value {
// 				max = value
// 			}
// 			if min > value {
// 				min = value
// 			}

// 		}
// 		current = value
// 	}

// 	return min, max
// }
