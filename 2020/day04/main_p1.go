package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	// "reflect"
	"strings"
)

func main() {
	input, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	nValid := 0

	for _, p := range input {
		if p.IsValid() {
			nValid++
		}
	}
	fmt.Println(nValid)
}

type PassportBatch []Passport

type Passport map[string]string

// type Passport struct {
// 	byr string
// 	iyr string
// 	eyr string
// 	hgt string
// 	hcl string
// 	ecl string
// 	pid string
// 	cid string
// }

func (p Passport) IsValid() bool {
	_, ok := p["cid"]

	return len(p) == 8 || (len(p) == 7 && !ok)

}

func readInput(path string) (PassportBatch, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()

	var passports PassportBatch

	currentPassport := Passport{}

	s := bufio.NewScanner(f)
	for s.Scan() {
		line := s.Text()
		if len(line) == 0 {
			// Delimiter for new passport. So:
			// Save current passport in db
			passports = append(passports, currentPassport)
			// Reinitialize passport
			currentPassport = Passport{}
			// Go to next iteration
			continue
		}
		fields := strings.Split(line, " ")
		for _, field := range fields {
			s := strings.Split(field, ":")
			key, value := s[0], s[1]
			currentPassport[key] = value

		}
	}
	passports = append(passports, currentPassport)

	return passports, s.Err()
}
