package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	tiles := parseInput("input_test.txt")
	// tiles := parseInput("input.txt")
	for k1, v1 := range tiles {
		found := 0
		for k2, v2 := range tiles {
			if k1 == k2 {
				continue
			}
			for p1, b1 := range v1.borders {
				for _, b2 := range v2.borders {
					if b1 == b2 || b1 == reverse(b2) {
						tiles[k1].neighbors[p1] = k2
						found++
					}
				}
			}

		}
	}

	// find corners
	corners := make(map[int]bool)
	for k, v := range tiles {
		count := 0
		for _, r := range v.neighbors {
			if r != -1 {
				count++
			}
		}
		if count == 2 {
			corners[k] = true
		}
	}
	for _, v := range tiles {
		fmt.Println(v.id, v.neighbors)
	}

	// start from a corner and arbritarily define it at the top left corner.
	// we'll build from there.
	var startId int
	for k, _ := range corners {
		startId = k
		break
	}

	// this will hold the full image
	fullImage := [][]int{[]int{startId}}

	// rotate corner to be correctly oriented
	for (tiles[startId].neighbors["left"] != -1) || (tiles[startId].neighbors["top"] != -1) {
		tiles[startId].rotate(1)
	}

	// fmt.Println(tiles[startId].data, "\n")
	// for _, s := range []string{"top", "bottom", "left", "right"} {
	// 	b := tiles[startId].getBorder(s)

	// 	fmt.Println(b, "\n")
	// }
	// keep track of the tiles we put into the image
	seen := make(map[int]bool)
	seen[startId] = true

	currentX := 0
	currentY := 0
	// for len(seen) != len(tiles) {
	z := 0
	for z < 1 {
		imId := fullImage[currentY][currentX]
		im := tiles[imId]
		// add images going right
		nextId := im.neighbors["right"]
		if nextId == -1 {
			// end of the line, go to start and get below
			break
		}
		next := tiles[nextId]
		imRightBorder := im.getBorder("right")
		nextLeftBorder := next.getBorder("left")
		fmt.Println(imRightBorder, "\n")
		i := 0
		j := 0
		for imRightBorder != nextLeftBorder {
			next.rotate(1)
			nextLeftBorder = next.getBorder("left")
			fmt.Println(nextLeftBorder, "\n")
			i++
			fmt.Println(i)
			if j == 0 && i >= 4 {
				// we did a full rotation, still no match, so
				// let's flip it and do it again.
				fmt.Println("Flipping")
				next.flip()
				j++

			} else if i >= 8 {
				fmt.Println("did not find any match")
				break
			}

		}
		z++
	}
}

type tile struct {
	id        int
	data      string
	neighbors map[string]int
	borders   map[string]string
}

// flip
func (t *tile) flip() {
	in := strings.Trim(t.data, "\n")
	var out strings.Builder

	lines := strings.Split(in, "\n")

	i := 0
	for i < len(lines) {

		j := len(lines[0]) - 1
		for j >= 0 {
			fmt.Fprint(&out, string(lines[i][j]))
			j--
		}
		if i != len(lines)-1 {
			fmt.Fprint(&out, "\n")
		}
		i++
	}
	// update tile data
	t.data = out.String()
	before := make(map[string]int)
	for k, v := range t.neighbors {
		before[k] = v
	}
	t.neighbors["right"] = before["left"]
	t.neighbors["left"] = before["right"]
}

// rotate tile data clockwise as many times as n
func (t *tile) rotate(n int) {
	in := t.data
	var out strings.Builder

	// fills in with original data
	original := strings.Split(in, "\n")
	for k, line := range original {
		for _, c := range line {
			fmt.Fprint(&out, string(c))
		}
		if k != len(original)-1 {
			fmt.Fprint(&out, "\n")
		}
	}

	borders := []string{"top", "right", "bottom", "left"}

	i := 0
	for i < n {

		in = strings.Trim(out.String(), "\n")
		out.Reset()

		lines := strings.Split(in, "\n")
		j := 0
		for j < len(lines[0]) {
			l := len(lines) - 1
			for l >= 0 {
				fmt.Fprint(&out, string(lines[l][j]))
				l--
			}
			if j != len(lines[0])-1 {
				fmt.Fprint(&out, "\n")
			}
			j++
		}
		i++

		// update tile data
		t.data = out.String()
		values := make([]int, 0)
		for _, v := range borders {
			values = append(values, t.neighbors[v])
		}
		for i, v := range borders {
			t.neighbors[v] = values[(i+1)%len(values)]
		}
	}
}

func (t *tile) getBorder(side string) string {
	data := strings.Split(t.data, "\n")
	var out string
	switch side {
	case "top":
		out = data[0]
	case "bottom":
		out = data[len(data)-1]
	case "left":
		var b strings.Builder
		for _, line := range data {
			fmt.Fprint(&b, string(line[0]))
		}
		out = b.String()
	case "right":
		var b strings.Builder
		for _, line := range data {
			fmt.Fprint(&b, string(line[len(line)-1]))
		}
		out = b.String()
	}
	return out
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
			"top":    topBorder,
			"right":  rightBorder.String(),
			"bottom": bottomBorder,
			"left":   leftBorder.String(),
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
