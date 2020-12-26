package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		log.Fatalln("Failed to read input:", err)
	}

	tiles := parse(string(data))

	orientation := findOrientation(tiles)

	fullImage := renderImage(orientation)
	var n int
	for i := 0; i < 2; i++ {
		for j := 0; j < 2; j++ {
			for k := 0; k < 3; k++ {
				if found := findMonsters(fullImage); found > 0 {
					n = strings.Count(fullImage.String(), "#") - (found * 15)
				}
				fullImage = fullImage.Rotate(90)
			}
			fullImage.Flip('y')
		}
		fullImage.Flip('x')
	}
	fmt.Println(n)
}

func parse(input string) []*tile {
	var tiles []*tile
	for _, tileData := range strings.Split(input, "\n\n") {
		tiles = append(tiles, newTile(tileData))
	}
	return tiles
}

func newTile(tileData string) *tile {
	t := tile{
		data: make(map[int]map[int]rune),
		xmax: 9,
		ymax: 9,
	}
	for i, line := range strings.Split(tileData, "\n") {
		if i == 0 {
			fmt.Sscanf(line, "Tile %d:", &t.id)
			continue
		}
		y := i - 1
		for x, r := range line {
			if _, ok := t.data[x]; !ok {
				t.data[x] = make(map[int]rune)
			}
			t.data[x][y] = r
		}
	}
	return &t
}

type tile struct {
	id         int64
	data       map[int]map[int]rune
	xmax, ymax int
}

func (t tile) String() string {
	var b strings.Builder
	b.WriteString(fmt.Sprintf("Tile %d:", t.id))
	for y := 0; y <= t.ymax; y++ {
		b.WriteRune('\n')
		for x := 0; x <= t.xmax; x++ {
			b.WriteRune(t.data[x][y])
		}
	}
	return b.String()
}

// Borders in clockwise order from top
func (t tile) Borders() []string {
	var top, bot strings.Builder
	for x := 0; x <= t.xmax; x++ {
		top.WriteRune(t.data[x][0])
		bot.WriteRune(t.data[t.xmax-x][9])
	}
	var left, right strings.Builder
	for y := 0; y <= t.ymax; y++ {
		left.WriteRune(t.data[0][t.ymax-y])
		right.WriteRune(t.data[9][y])
	}
	return []string{top.String(), right.String(), bot.String(), left.String()}
}

// Rotate clockwise
func (t *tile) Rotate(angle int) *tile {
	newData := make(map[int]map[int]rune)
	for x := 0; x <= t.xmax; x++ {
		newData[x] = make(map[int]rune)
	}
	switch angle {
	case 0, 360:
		return t
	case 90:
		// 0,0 => 9,0
		for x := 0; x <= t.xmax; x++ {
			for y := 0; y <= t.ymax; y++ {
				newData[t.ymax-y][x] = t.data[x][y]
			}
		}
	case 180:
		// 0,0 => 9,9
		for x := 0; x <= t.xmax; x++ {
			for y := 0; y <= t.ymax; y++ {
				newData[t.xmax-x][t.ymax-y] = t.data[x][y]
			}
		}
	case 270:
		// 0,0 => 0,9
		for x := 0; x <= t.xmax; x++ {
			for y := 0; y <= t.ymax; y++ {
				newData[y][t.xmax-x] = t.data[x][y]
			}
		}
	}
	t.data = newData
	return t
}

// Flip a tile about x/y axis
func (t *tile) Flip(axis rune) *tile {
	newData := make(map[int]map[int]rune)
	for x := 0; x <= t.xmax; x++ {
		newData[x] = make(map[int]rune)
	}
	switch axis {
	case 'x':
		for x := 0; x <= t.xmax; x++ {
			for y := 0; y <= t.ymax; y++ {
				newData[t.xmax-x][y] = t.data[x][y]
			}
		}
	case 'y':
		for x := 0; x <= t.xmax; x++ {
			for y := 0; y <= t.ymax; y++ {
				newData[x][t.ymax-y] = t.data[x][y]
			}
		}
	}
	t.data = newData
	return t
}

// Trim the borders from a tile
func (t *tile) Trim() *tile {
	newData := make(map[int]map[int]rune)
	for x := 0; x < t.xmax-1; x++ {
		newData[x] = make(map[int]rune)
	}
	for x := 1; x < t.xmax; x++ {
		for y := 1; y < t.ymax; y++ {
			newData[x-1][y-1] = t.data[x][y]
		}
	}
	t.data = newData
	return t
}

func findOrientation(tiles []*tile) map[int]map[int]*tile {
	orientation := make(map[int]map[int]*tile)
	size := int(math.Sqrt(float64(len(tiles))))
	// If we use a grid twice as big as required then no matter which one we start
	// with there will be room for all the tiles
	for x := 0; x < size*2; x++ {
		orientation[x] = make(map[int]*tile)
	}
	// Set the first one in the middle
	x, y := size, size
	orientation[x][y] = tiles[0]
	// Delete from slice
	tiles = tiles[1:]
outer:
	for {
		if len(tiles) == 0 {
			break
		}
		for x := range orientation {
			for y, t := range orientation[x] {
				for i, b := range t.Borders() {
					// Need to reverse to spot the same edge combo
					r := reverse(b)
					for idx, candidate := range tiles {
						for j, c := range candidate.Borders() {
							if r == c || b == c {
								// Calculate if a rotation is required
								diff := j - i
								switch diff {
								case 0:
									candidate.Rotate(180)
								case 2, -2:
									// no rotation
								case -1, 3:
									candidate.Rotate(270)
								case 1, -3:
									candidate.Rotate(90)
								}
								switch i {
								case 0:
									// If b == c need to flip as well as rotate
									if b == c && r != c {
										candidate.Flip('x')
									}
									orientation[x][y-1] = candidate
								case 1:
									if b == c && r != c {
										candidate.Flip('y')
									}
									orientation[x+1][y] = candidate
								case 2:
									if b == c && r != c {
										candidate.Flip('x')
									}
									orientation[x][y+1] = candidate
								case 3:
									if b == c && r != c {
										candidate.Flip('y')
									}
									orientation[x-1][y] = candidate
								}
								// Remove from candidates
								tiles = append(tiles[:idx], tiles[idx+1:]...)
								continue outer
							}
						}
					}
				}
			}
		}
	}
	return orientation
}

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func cornerProduct(orientation map[int]map[int]*tile) int64 {
	var xmax, ymax int
	xmin, ymin := math.MaxInt16, math.MaxInt16
	for x := range orientation {
		for y, t := range orientation[x] {
			if t != nil {
				if x < xmin {
					xmin = x
				}
				if x > xmax {
					xmax = x
				}
				if y < ymin {
					ymin = y
				}
				if y > ymax {
					ymax = y
				}
			}
		}
	}

	return orientation[xmin][ymin].id * orientation[xmin][ymax].id * orientation[xmax][ymin].id * orientation[xmax][ymax].id
}

func renderImage(orientation map[int]map[int]*tile) *tile {
	var xmax, ymax int
	xmin, ymin := math.MaxInt16, math.MaxInt16
	for x := range orientation {
		for y, t := range orientation[x] {
			if t != nil {
				if x < xmin {
					xmin = x
				}
				if x > xmax {
					xmax = x
				}
				if y < ymin {
					ymin = y
				}
				if y > ymax {
					ymax = y
				}
			}
		}
	}

	image := &tile{data: make(map[int]map[int]rune)}
	for ox := xmin; ox <= xmax; ox++ {
		for oy := ymin; oy <= ymax; oy++ {
			tile := orientation[ox][oy].Trim()
			for tx := 0; tx < 8; tx++ {
				for ty := 0; ty < 8; ty++ {
					ix, iy := tx+(ox-xmin)*8, ty+(oy-ymin)*8
					if _, ok := image.data[ix]; !ok {
						image.data[ix] = make(map[int]rune)
					}
					image.data[ix][iy] = tile.data[tx][ty]
					if ix > image.xmax {
						image.xmax = ix
					}
					if iy > image.ymax {
						image.ymax = iy
					}
				}
			}
		}
	}

	return image
}

func findMonsters(t *tile) int {
	var found int

	// Search the grid for possible starts of seamonsters
	for x := 0; x <= t.xmax-18; x++ {
	nextTile:
		for y := 0; y < t.ymax-2; y++ {
			for sx := range monsterShape.data {
				for sy := range monsterShape.data[sx] {
					if r, ok := t.data[x+sx][y+sy]; ok && r == '#' {
						continue
					}
					continue nextTile
				}
			}
			found++
		}
	}

	return found
}

var monsterShape = tile{
	data: map[int]map[int]rune{
		0: {
			1: '#',
		},
		1: {
			2: '#',
		},
		4: {
			2: '#',
		},
		5: {
			1: '#',
		},
		6: {
			1: '#',
		},
		7: {
			2: '#',
		},
		10: {
			2: '#',
		},
		11: {
			1: '#',
		},
		12: {
			1: '#',
		},
		13: {
			2: '#',
		},
		16: {
			2: '#',
		},
		17: {
			1: '#',
		},
		18: {
			0: '#',
			1: '#',
		},
		19: {
			1: '#',
		},
	},
}
