const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

const isLower = character => {
  return (character === character.toLowerCase()) && (character !== character.toUpperCase());
}

const isUpper = character => {
  return (character !== character.toLowerCase()) && (character === character.toUpperCase());
}

const crawl = input => {
  let output = input
  let start = 0
  let goOn=true
  while (goOn) {
    let match = false
    for (let i=start; i<output.length-1; i++){
      const letter = output[i]
      if (isLower(letter)) {
        const nextLetter = output[i+1]
        if (isUpper(nextLetter) && nextLetter.toLowerCase()==letter) {
          output = output.slice(0, i) + output.slice(i+2, output.length)
          match = true
          start = i == 0 ? 0 : i-1
          break;
        }
      } else if (isUpper(letter)) {
        const nextLetter = output[i+1]
        if (isLower(nextLetter) && nextLetter.toUpperCase()==letter) {
          output = output.slice(0, i) + output.slice(i+2, output.length)
          match = true
          start = i == 0 ? 0 : i-1
          break;
        }
      }
    }
    if (match == false) goOn=false
  }
  return output.length
} 

const ex = 'dabAcCaCBAcCcaDA'

console.assert(crawl(ex) == 10)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', crawl(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const removeUnit = (input, unit) => {
  let output = input
  let goOn=true
  while (goOn) {
    let match = false
    for (let i=0; i<output.length; i++){
      const letter = output[i]
      if (letter.toLowerCase() === unit) {
          output = output.slice(0, i) + output.slice(i+1, output.length)
          match = true
          break;
      }
    }
    if (match == false) goOn=false
  }    
  return output
}

const getAllLetters = input => new Set(input.split('').map(d => d.toLowerCase()))


const testEachUnit = input => {
  const allUnits = getAllLetters(input)
  const result = {}
  for (const letter of allUnits) {
    const newInput = removeUnit(input, letter)
    result[letter] = crawl(newInput)
  }
  const mini = Object.keys(result).reduce((a, b) => result[a] < result[b] ? a : b);
  return result[mini]
}

console.assert(testEachUnit(ex) == 4)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', testEachUnit(data))
});



