package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

func main() {
	lines, err := readInput("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	memory := make(map[string]int64)
	currentMask := ""

	for _, line := range lines {
		if strings.Contains(line, "mask") {
			currentMask = strings.Replace(line, "mask = ", "", -1)
			// go to next line
			continue
		}

		parts := strings.Split(line, " = ")

		addr_s := strings.Replace(strings.Split(parts[0], "mem[")[1], "]", "", -1)
		addr, _ := strconv.Atoi(addr_s)
		n_s := parts[1]
		n, _ := strconv.Atoi(n_s)
		// make it pad with zeros up to 36 bytes
		binary := fmt.Sprintf("%036b", addr)

		m := ""
		countX := 0
		for i, bit := range currentMask {
			switch bit {
			case '1':
				m += "1"
			case '0':
				m += string([]byte(binary)[i])
			case 'X':
				m += "X"
				countX++
			}
		}

		for i := 0; i < int(math.Pow(2, float64(countX))); i++ {
			combination := fmt.Sprintf("%b", i)
			for len(combination) < countX {
				combination = "0" + combination
			}
			tempMasked := m
			for _, bit := range combination {
				tempMasked = strings.Replace(tempMasked, "X", string(bit), 1)
			}
			memory[tempMasked] = int64(n)
		}
	}

	var total int64
	for _, v := range memory {
		total += v
	}

	fmt.Println(total)

}

func readInput(path string) ([]string, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return []string{}, fmt.Errorf("could not open %s: %v", path, err)
	}

	instructions := strings.Split(strings.TrimSpace(string(f)), "\n")

	return instructions, nil
}
