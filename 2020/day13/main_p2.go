package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math/big"
	"strconv"
	"strings"
)

func main() {
	note, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	m := make(map[int]int)
	for _, b := range note.ids {
		m[b.schedule] = (b.schedule - b.index) % b.schedule
		if m[b.schedule] < 0 {
			m[b.schedule] = m[b.schedule] + b.schedule
		}
	}

	a, n := make([]*big.Int, 0), make([]*big.Int, 0)
	for k, v := range m {
		kb := big.NewInt(int64(k))
		vb := big.NewInt(int64(v))
		a = append(a, vb)
		n = append(n, kb)
	}
	fmt.Println(crt(a, n))
}

type bus struct {
	index, schedule int
}

type notes struct {
	time int
	ids  []bus
}

// Chinese Remainder code taken from RosettaCode
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Go
func crt(a, n []*big.Int) (*big.Int, error) {
	one := big.NewInt(1)
	p := new(big.Int).Set(n[0])
	for _, n1 := range n[1:] {
		p.Mul(p, n1)
	}
	var x, q, s, z big.Int
	for i, n1 := range n {
		q.Div(p, n1)
		z.GCD(nil, &s, n1, &q)
		if z.Cmp(one) != 0 {
			return nil, fmt.Errorf("%d not coprime", n1)
		}
		x.Add(&x, s.Mul(a[i], s.Mul(&s, &q)))
	}
	return x.Mod(&x, p), nil
}

func readInput(path string) (notes, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return notes{}, fmt.Errorf("could not open %s: %v", path, err)
	}

	lines := strings.Split(string(f), "\n")
	l := lines[1]
	var ids []bus
	for n, s := range strings.Split(l, ",") {
		if s != "x" {
			i, _ := strconv.Atoi(s)
			ids = append(ids, bus{n, i})
		}
	}
	j, _ := strconv.Atoi(lines[0])
	n := notes{j, ids}
	return n, nil
}
