package main

import (
	"fmt"
	"math"
)

type GameState struct {
	PlayerHitPoints      int
	PlayerMana           int
	PlayerSpent          int
	PlayerEffectShield   int
	PlayerEffectPoison   int
	PlayerEffectRecharge int
	BossHitPoints        int
	BossDamage           int
	TurnDamage           int // part 2
}

func (gs *GameState) won() bool {
	return gs.PlayerHitPoints > 0 && gs.BossHitPoints <= 0
}

func (gs *GameState) lost() bool {
	return gs.PlayerHitPoints <= 0
}

func (gs *GameState) spend(mana int) bool {
	if gs.PlayerMana < mana {
		return false
	}
	gs.PlayerMana -= mana
	gs.PlayerSpent += mana
	return true
}

func (gs *GameState) armor() int {
	if gs.PlayerEffectShield > 0 {
		return 7
	}
	return 0
}

func (gs *GameState) update() {
	if gs.PlayerEffectPoison > 0 {
		gs.BossHitPoints -= 3
	}
	if gs.PlayerEffectRecharge > 0 {
		gs.PlayerMana += 101
	}

	if gs.PlayerEffectShield > 0 {
		gs.PlayerEffectShield--
	}
	if gs.PlayerEffectPoison > 0 {
		gs.PlayerEffectPoison--
	}
	if gs.PlayerEffectRecharge > 0 {
		gs.PlayerEffectRecharge--
	}
}

type Spell func(gs GameState) (GameState, bool)

func MagicMissile(gs GameState) (GameState, bool) {
	if !gs.spend(53) {
		return gs, false
	}
	gs.BossHitPoints -= 4
	return gs, true
}

func Drain(gs GameState) (GameState, bool) {
	if !gs.spend(73) {
		return gs, false
	}
	gs.PlayerHitPoints += 2
	gs.BossHitPoints -= 2
	return gs, true
}

func Shield(gs GameState) (GameState, bool) {
	if gs.PlayerEffectShield > 0 || !gs.spend(113) {
		return gs, false
	}
	gs.PlayerEffectShield = 6
	return gs, true
}

func Poison(gs GameState) (GameState, bool) {
	if gs.PlayerEffectPoison > 0 || !gs.spend(173) {
		return gs, false
	}
	gs.PlayerEffectPoison = 6
	return gs, true
}

func Recharge(gs GameState) (GameState, bool) {
	if gs.PlayerEffectRecharge > 0 || !gs.spend(229) {
		return gs, false
	}
	gs.PlayerEffectRecharge = 5
	return gs, true
}
func main() {
	init := GameState{PlayerHitPoints: 50, PlayerMana: 500, BossHitPoints: 71, BossDamage: 10}
	best := GameState{PlayerSpent: math.MaxInt32}

	var PlayerTurn, BossTurn func(GameState)

	PlayerTurn = func(gs GameState) {
		// part 2: hard mode
		gs.PlayerHitPoints -= gs.TurnDamage
		if gs.lost() {
			return
		}

		gs.update()
		if gs.won() {
			if gs.PlayerSpent < best.PlayerSpent {
				best = gs
			}
			return
		}
		for _, spell := range []Spell{MagicMissile, Drain, Shield, Poison, Recharge} {
			if newGs, ok := spell(gs); ok && newGs.PlayerSpent < best.PlayerSpent {
				BossTurn(newGs)
			}
		}
	}

	BossTurn = func(gs GameState) {
		gs.update()
		if gs.won() {
			if gs.PlayerSpent < best.PlayerSpent {
				best = gs
			}
			return
		}
		damage := gs.BossDamage - gs.armor()
		if damage <= 0 {
			damage = 1
		}
		gs.PlayerHitPoints -= damage
		if !gs.lost() {
			PlayerTurn(gs)
		}
	}

	PlayerTurn(init)
	p1 := best.PlayerSpent

	init.TurnDamage = 1
	best = GameState{PlayerSpent: math.MaxInt32}
	PlayerTurn(init)
	p2 := best.PlayerSpent

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
