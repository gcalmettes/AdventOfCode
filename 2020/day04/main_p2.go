package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
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

func (p Passport) IsValid() bool {
	_, ok_cid := p["cid"]
	byr, ok_byr := p["byr"]
	iyr, ok_iyr := p["iyr"]
	eyr, ok_eyr := p["eyr"]
	hgt, ok_hgt := p["hgt"]
	hcl, ok_hcl := p["hcl"]
	ecl, ok_ecl := p["ecl"]
	pid, ok_pid := p["pid"]

	isValid := len(p) == 8 || (len(p) == 7 && !ok_cid)
	fmt.Println()
	fmt.Println(isValid, ok_cid)
	if ok_byr {
		i, err := strconv.Atoi(byr)
		if err != nil {
			isValid = false
		} else {

			isValid = isValid && (len(byr) == 4 && (i >= 1920 && i <= 2002))
		}
		fmt.Println(byr, isValid)
	}

	if ok_iyr {
		i, err := strconv.Atoi(iyr)
		if err != nil {
			isValid = false
		} else {
			isValid = isValid && (len(iyr) == 4 && (i >= 2010 && i <= 2020))

		}
		fmt.Println(iyr, isValid)
	}
	if ok_eyr {
		i, err := strconv.Atoi(eyr)
		if err != nil {
			isValid = false
		} else {
			isValid = isValid && (len(eyr) == 4 && (i >= 2020 && i <= 2030))

		}
		fmt.Println(eyr, isValid)
	}
	if ok_hgt {
		var n int
		var unit string
		fmt.Sscanf(hgt, "%d%s", &n, &unit)
		if unit == "cm" {
			isValid = isValid && (n >= 150 && n <= 193)
		} else if unit == "in" {
			isValid = isValid && (n >= 59 && n <= 76)
		} else {
			isValid = false
		}
		fmt.Println(hgt, isValid)
	}
	if ok_ecl {
		set := make(map[string]bool)
		for _, v := range []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"} {
			set[v] = true
		}

		isValid = isValid && set[ecl]
		fmt.Println(ecl, isValid)
	}
	if ok_pid {
		if _, err := strconv.Atoi(pid); err != nil {
			isValid = false
		}
		isValid = isValid && (len(pid) == 9)
		fmt.Println(pid, isValid)
	}
	if ok_hcl {
		isValid = isValid && strings.HasPrefix(hcl, "#")
		if isValid {
			re := regexp.MustCompile(`[0-9a-f]{5}`)
			hcl = hcl[1:]
			isValid = isValid && len(hcl) == 6
			isValid = isValid && re.Match([]byte(hcl))
		}
		fmt.Println(hcl, isValid)
	}
	return isValid

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
