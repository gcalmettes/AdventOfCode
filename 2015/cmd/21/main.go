package main

import (
	"fmt"
)

type Item struct {
	Name   string
	Cost   int
	Damage int
	Armor  int
}

type Player struct {
	HitPoints int
	Damage    int
	Armor     int
}

func (p *Player) IsWinnerAgainst(o *Player) bool {
	playerLife := p.HitPoints
	opponentLife := o.HitPoints

	opponentTurnDamage := p.Damage - o.Armor
	if opponentTurnDamage <= 0 {
		opponentTurnDamage = 1
	}
	playerTurnDamage := o.Damage - p.Armor
	if playerTurnDamage <= 0 {
		playerTurnDamage = 1
	}
	for playerLife > 0 && opponentLife > 0 {
		opponentLife -= opponentTurnDamage
		if opponentLife <= 0 {
			break
		}
		playerLife -= playerTurnDamage
	}
	return playerLife > 0
}

func main() {

	WEAPONS := []Item{
		{"Dagger", 8, 4, 0},
		{"Shortsword", 10, 5, 0},
		{"Warhammer", 25, 6, 0},
		{"Longsword", 40, 7, 0},
		{"Greataxe", 74, 8, 0},
	}
	ARMOR := []Item{
		{"Leather", 13, 0, 1},
		{"Chainmail", 31, 0, 2},
		{"Splintmail", 53, 0, 3},
		{"Bandedmail", 75, 0, 4},
		{"Platemail", 102, 0, 5},
	}
	RINGS := []Item{
		{"Damage +1", 25, 1, 0},
		{"Damage +2", 50, 2, 0},
		{"Damage +3", 100, 3, 0},
		{"Defense +1", 20, 0, 1},
		{"Defense +2", 40, 0, 2},
		{"Defense +3", 80, 0, 3},
	}

	boss := Player{104, 8, 1}

	wins := []int{}
	losses := []int{}
	for _, w := range WEAPONS {
		// without optional armor
		for _, r1 := range RINGS {
			price := w.Cost + r1.Cost
			me := Player{100, w.Damage + r1.Damage, w.Armor + r1.Armor}
			if me.IsWinnerAgainst(&boss) {
				wins = append(wins, price)
			} else {
				losses = append(losses, price)
			}
			for _, r2 := range RINGS {
				if r1 == r2 {
					continue
				}
				price := w.Cost + r1.Cost + r2.Cost
				me := Player{100, w.Damage + r1.Damage + r2.Damage, w.Armor + r1.Armor + r2.Armor}
				if me.IsWinnerAgainst(&boss) {
					wins = append(wins, price)
				} else {
					losses = append(losses, price)
				}
			}
		}

		// with optional armor
		for _, a := range ARMOR {
			price := w.Cost + a.Cost
			me := Player{100, w.Damage + a.Damage, w.Armor + a.Armor}
			if me.IsWinnerAgainst(&boss) {
				wins = append(wins, price)
			} else {
				losses = append(losses, price)
			}
			for _, r1 := range RINGS {
				price := w.Cost + a.Cost + r1.Cost
				me := Player{100, w.Damage + a.Damage + r1.Damage, w.Armor + a.Armor + r1.Armor}
				if me.IsWinnerAgainst(&boss) {
					wins = append(wins, price)
				} else {
					losses = append(losses, price)
				}
				for _, r2 := range RINGS {
					if r1 == r2 {
						continue
					}
					price := w.Cost + a.Cost + r1.Cost + r2.Cost
					me := Player{100, w.Damage + a.Damage + r1.Damage + r2.Damage, w.Armor + a.Armor + r1.Armor + r2.Armor}
					if me.IsWinnerAgainst(&boss) {
						wins = append(wins, price)
					} else {
						losses = append(losses, price)
					}
				}
			}
		}
	}

	var p1, p2 int

	for _, w := range wins {
		if p1 == 0 {
			p1 = w
		}
		if -p1 < -w {
			p1 = w
		}
	}
	for _, l := range losses {
		if l > p2 {
			p2 = l
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
