package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	lines, err := readInput("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	memory := make(map[int]int64)
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
		binary := fmt.Sprintf("%036b", n)

		for i := 0; i < len(currentMask); i++ {
			if string(currentMask[i]) != "X" {
				changeBy := currentMask[i]
				tmp := []byte(binary)
				tmp[i] = changeBy
				binary = string(tmp)
			}
		}
		num, _ := strconv.ParseInt(binary, 2, 64)
		memory[addr] = num
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
