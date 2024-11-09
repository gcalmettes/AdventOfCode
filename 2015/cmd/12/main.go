package main

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func GetNumbers(s string) []int {
	re := regexp.MustCompile("(-?\\d+)")
	numbers := []int{}
	for _, m := range re.FindAllStringSubmatch(s, -1) {
		n, err := strconv.Atoi(m[0])
		if err != nil {
			panic(err)
		}
		numbers = append(numbers, n)
	}
	return numbers
}

func RemoveRed(a interface{}) interface{} {
	if obj, ok := a.([]interface{}); ok == true {
		li := []interface{}{}
		for _, o := range obj {
			li = append(li, RemoveRed(o))
		}
		return li
	}

	if obj, ok := a.(map[string]interface{}); ok == true {
		mp := map[interface{}]interface{}{}
		valid := true
		for k, v := range obj {
			if s, _ := v.(string); s == "red" {
				valid = false
			}
			mp[k] = RemoveRed(v)
		}
		if valid {
			return mp
		} else {
			return nil
		}
	}
	return a
}

func RemoveAllRed(s string) string {
	var jsonData interface{}
	json.Unmarshal([]byte(s), &jsonData)
	cleaned := RemoveRed(jsonData)
	return fmt.Sprintf("%#v", cleaned)
}

func main() {
	input, err := os.ReadFile("./inputs/12.txt")
	if err != nil {
		panic(err)
	}

	var p1, p2 int

	for _, n := range GetNumbers(string(input)) {
		p1 += n
	}

	s2 := RemoveAllRed(string(input))
	for _, n := range GetNumbers(s2) {
		p2 += n
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
