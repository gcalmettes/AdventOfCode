const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

const findMatches = (data) => {
  return data.reduce((acc, val) => {
      const count = {}
      for (letter of val) {
        count[letter] = count[letter] ? count[letter]+1 : 1
      }
      
      const matches = Object.values(count).reduce((a, b) => {
        if (b == 2) a[0]=1
        if (b == 3) a[1]=1
        return a
      }, [0, 0])

      acc[0]+=matches[0]
      acc[1]+=matches[1]
      return acc

    }, [0, 0])
}

const getChecksum = (data) => {
  const matches = findMatches(data)
  return matches[0]*matches[1]
}

const ex = ['abcdef', 'bababc', 'abbcde', 'abcccd', 'aabcdd', 'abcdee', 'ababab']
console.assert(getChecksum(ex) == 12)


fs.readFile(filename, 'utf8', (err, file) => {
  if (err) throw err;
  data = file.trim().split('\n')
  console.log('part 1:', getChecksum(data))
  
});

/////////////////////////////
// part 2
/////////////////////////////

const getAllpairs = (array) => {
  let pairs = []
  for (let i=0; i<array.length; i++) {
    for (let j=i+1; j<array.length; j++){
      pairs.push([array[i], array[j]])
    }
  }
  return pairs
}

const difference = (pair) => {
  const [p1, p2] = pair.map(d => d.split(''))
  let count = 0
  for (let i=0; i<p1.length; i++){
    count = p1[i] == p2[i] ? count : count+1
  }
  return count
}

const getBoxes = (data) => {
  const allPairs = getAllpairs(data)
  return allPairs.reduce((acc, pair) => {
    const diff = difference(pair)
    return diff != 1 ? acc : pair[0].split('').filter(l => pair[1].split('').includes(l)).join('')
  }, null)
}

const ex2 = ['abcde', 'fghij', 'klmno', 'pqrst', 'fguij','axcye', 'wvxyz']
console.assert(getBoxes(ex2) == 'fgij')

fs.readFile(filename, 'utf8', (err, file) => {
  if (err) throw err;
  data = file.trim().split('\n')
  console.log('part 2:', getBoxes(data))
  
});


