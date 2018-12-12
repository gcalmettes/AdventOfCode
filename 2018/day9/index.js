const getGame = input => {
  const { nPlayers, nMarbles } = /(?<nPlayers>\d+) players; last marble is worth (?<nMarbles>\d+) points/.exec(input).groups
  // const [nPlayers, nMarbles] = input.match(/\d+/g).map(Number);
  return { nPlayers: Number(nPlayers), nMarbles: Number(nMarbles) }
}

const insertMarbleAfter = (marble, points) => {
    const marbleToAdd = {
        points,
        prev: marble,
        next: marble.next,
    };
    marble.next.prev = marbleToAdd;
    marble.next = marbleToAdd;
    return marbleToAdd;
};

// rewrote for part 2 for performance issues.
// linked list implementation, need insertion to be O(1)
const playGame = input => {
    const { nPlayers, nMarbles } = getGame(input)
    
    const scores = Array.from({length: nPlayers})
    .reduce((acc, cur, i) => {
      acc[i+1] = 0
      return acc
    }, {})
    let currentPlayer = 1;
    
    // first marble is object with self reference
    let current = new function foo(){
        this.points= 0
        this.prev = this
        this.next= this
    };
    
    for (let m = 1; m <= nMarbles; m += 1) {
        if (m % 23 === 0) {
            scores[currentPlayer] += m;
            current = current.prev.prev.prev.prev.prev.prev;
            scores[currentPlayer] += current.prev.points;
            current.prev.prev.next = current;
            current.prev = current.prev.prev;
        } else {
            current = insertMarbleAfter(current.next, m);
        }
        currentPlayer = currentPlayer % nPlayers + 1;
    }
    return Math.max(...Object.values(scores));
};

const ex1 = '10 players; last marble is worth 1618 points: high score is 8317';
const ex2 = '13 players; last marble is worth 7999 points: high score is 146373';
const ex3 = '17 players; last marble is worth 1104 points: high score is 2764';
const ex4 = '21 players; last marble is worth 6111 points: high score is 54718';
const ex5 = '30 players; last marble is worth 5807 points: high score is 37305';

Array.from([ex1, ex2, ex3, ex4, ex5]).forEach(d => {
  const toMatch = +d.match(/\d+ players; last marble is worth \d+ points: high score is (\d+)/)[1]
  console.assert(playGame(d) == toMatch)
})

const INPUT = '410 players; last marble is worth 72059 points'
console.log('part 1:', playGame(INPUT))

const INPUT2 = '410 players; last marble is worth 7205900 points'
console.log('part 2:', playGame(INPUT2))
