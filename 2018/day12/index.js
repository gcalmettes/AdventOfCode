const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

const processInput = input => {
  
  const initialState = /initial state: (?<pots>[#\.]+)/.exec(input).groups.pots

  const rules = {}
  const pattern = /(?<pattern>[#\.]{5}) => (?<next>[#\.])/g
  while(match = pattern.exec(input)) {
    rules[match.groups.pattern] = match.groups.next
  }
  return { initialState, rules }
}


const ex = `initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #`

const simulateGenerations = (input, n) => {
  let { initialState, rules } = processInput(input)
  let newGeneration = initialState
  let firstPotIdx = 0
  for (let i=0; i<n; i++){
    // extend only if necessary
    let previous =  '..' + newGeneration + '..'
    newGeneration = '..'
    for (let p=2; p<previous.length-2; p++){
      const pattern = previous.slice(p-2, p+3)
      if (rules[pattern]) {
        newGeneration+=rules[pattern]
      } else {
        newGeneration+='.'//previous[p]
      }
    }
    newGeneration+='..'
    firstPotIdx-=2
  }
  let sum = 0
  for (let i=0,f=firstPotIdx; i<newGeneration.length; i++,f++){
    if (newGeneration[i]=='#') sum+=f
  }
  return sum
}

console.assert(simulateGenerations(ex, 20)==325)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', simulateGenerations(data, 20))
});


/////////////////////////////
// part 2
/////////////////////////////

const getStabilizedGrowing = (input, n) => {
  let { initialState, rules } = processInput(input)
  let newGeneration = initialState
  let firstPotIdx = 0
  
  // variable to check growth
  let prevSum = 0
  let diff = 0
  let prevDiff = 0
  let generation
  let deltaGrowth

  let sum = 0
  for (let i=0; i<n; i++){
    let previous =  '..' + newGeneration + '..'
    newGeneration = '..'
    for (let p=2; p<previous.length-2; p++){
      const pattern = previous.slice(p-2, p+3)
      if (rules[pattern]) {
        newGeneration+=rules[pattern]
      } else {
        newGeneration+='.'
      }
    }
    newGeneration+='..'
    firstPotIdx-=2

    sum = 0
    for (let i=0,f=firstPotIdx; i<newGeneration.length; i++,f++){
      if (newGeneration[i]=='#') sum+=f
    }
    
    if (sum-prevSum==diff && sum-prevSum==prevDiff) {
      console.log(`Stabilized at generation #${i+1} with score of ${sum}`)
      console.log(`Delta growth is ${diff}`)
      generation = i+1
      break
    }
    prevDiff = diff
    diff = sum-prevSum
    prevSum=sum
    deltaGrowth = diff
  }
  return [generation, sum, deltaGrowth]
}

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  const n = 50000000000
  const [generation, score, deltaGrowth] = getStabilizedGrowing(data, n)
  
  console.log('part 2:', score + (n-generation) * deltaGrowth)
});