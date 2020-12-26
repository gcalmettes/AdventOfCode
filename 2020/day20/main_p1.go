package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	tiles := parseInput("input.txt")
	for k1, v1 := range tiles {
		for k2, v2 := range tiles {
			if k1 == k2 {
				continue
			}
			found := 0
			for p1, b1 := range v1.borders {
				for p2, b2 := range v2.borders {
					if b1 == b2 {
						_ = p2
						tiles[k1].neighbors[p1] = k2
						found++
					}
				}
			}

		}
	}

	// NOTE: here I am not even constructing the image, just
	// finding which tile is a corner since we only need to provide them.
	total := 1
	for k, v := range tiles {
		count := 0
		for _, r := range v.neighbors {
			if r != -1 {
				count++
			}
		}
		// Because we check both in regular and reversed order
		// each border should match twice. So the corder (2 border matching)
		// will get 4 matches in total.
		// The  sides will match 6 and the other will match 8.
		if count == 4 {
			total *= k
		}
	}
	fmt.Println(total)
}

type tile struct {
	id        int
	data      string
	neighbors map[string]int
	borders   map[string]string
}

func NewTile(id int, data string) tile {
	slices := strings.Split(strings.Trim(data, "\n"), "\n")
	topBorder := slices[0]
	bottomBorder := slices[len(slices)-1]
	var leftBorder strings.Builder
	var rightBorder strings.Builder
	for _, s := range slices {
		first := string(s[0])
		last := string(s[len(s)-1])
		fmt.Fprint(&leftBorder, first)
		fmt.Fprint(&rightBorder, last)
	}
	t := tile{
		id,
		data,
		map[string]int{
			"top":    -1,
			"right":  -1,
			"bottom": -1,
			"left":   -1,
		},
		map[string]string{
			"top":     topBorder,
			"right":   rightBorder.String(),
			"bottom":  bottomBorder,
			"left":    leftBorder.String(),
			"topR":    reverse(topBorder),
			"rightR":  reverse(rightBorder.String()),
			"bottomR": reverse(bottomBorder),
			"leftR":   reverse(leftBorder.String()),
		},
	}
	return t
}

func (t *tile) String() string {
	var s strings.Builder
	fmt.Fprint(&s, *t)
	return s.String()
}

func reverse(s string) string {
	n := len(s)
	runes := make([]rune, n)
	for _, rune := range s {
		n--
		runes[n] = rune
	}
	return string(runes[n:])
}

func parseInput(path string) map[int]*tile {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not read file: %v", err)
	}

	tiles := strings.Split(string(f), "\n\n")

	formattedTiles := make(map[int]*tile)

	for _, t := range tiles {
		parts := strings.SplitN(t, "\n", 2)
		var id int
		fmt.Sscanf(parts[0], "Tile %d:", &id)
		imageData := parts[1]
		im := NewTile(id, imageData)
		formattedTiles[id] = &im
	}
	return formattedTiles
}
