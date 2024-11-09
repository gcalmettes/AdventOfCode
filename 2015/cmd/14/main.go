package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Reindeer struct {
	Name     string
	FlySpeed int
	FlyTime  int
	RestTime int
}

func (r *Reindeer) FlyDistance(t int) int {
	rest := t % (r.FlyTime + r.RestTime)
	if rest > 0 && rest < r.FlyTime+1 {
		return r.FlySpeed
	} else {
		return 0
	}
}

func NewReindeer(name string, fs, ft, rt int) *Reindeer {
	return &Reindeer{
		Name:     name,
		FlySpeed: fs,
		FlyTime:  ft,
		RestTime: rt,
	}
}

func ParseLine(s string) []string {
	re := regexp.MustCompile("(.+) can fly (\\d+) km/s for (\\d+) seconds, but then must rest for (\\d+) seconds.")
	return re.FindStringSubmatch(s)[1:]
}

func main() {
	input, err := os.ReadFile("./inputs/14.txt")
	if err != nil {
		panic(err)
	}

	reindeers := []Reindeer{}
	reindeersDistance := map[string][2]int{}

	for _, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		parsed := ParseLine(line)
		fs, _ := strconv.Atoi(parsed[1])
		ft, _ := strconv.Atoi(parsed[2])
		rt, _ := strconv.Atoi(parsed[3])
		reindeers = append(reindeers, *NewReindeer(parsed[0], fs, ft, rt))
		reindeersDistance[parsed[0]] = [2]int{0, 0}
	}

	var p1, p2 int

	for i := range 2503 {
		for _, r := range reindeers {
			reindeersDistance[r.Name] = [2]int{reindeersDistance[r.Name][0] + r.FlyDistance(i+1), reindeersDistance[r.Name][1]}
		}
		leaderScore := 0
		leaders := ""
		for name, d := range reindeersDistance {
			if d[0] > leaderScore {
				leaderScore = d[0]
				leaders = name
			} else if d[0] == leaderScore {
				leaders = leaders + " " + name
			}
		}

		for _, r := range reindeers {
			if strings.Contains(leaders, r.Name) {
				reindeersDistance[r.Name] = [2]int{reindeersDistance[r.Name][0], reindeersDistance[r.Name][1] + 1}
			}
		}
	}

	for _, d := range reindeersDistance {
		if d[0] > p1 {
			p1 = d[0]
		}
		if d[1] > p2 {
			p2 = d[1]
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
