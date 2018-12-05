const fs = require('fs')

const filename = process.argv[2];

const processInput = (data) => data.trim(). split('\n').map(d => Number(d))

/////////////////////////////
// part 1
/////////////////////////////

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  res = processInput(data)
    .reduce((acc, val) => acc+val, 0)
  
  console.log('part 1:', res)
});

/////////////////////////////
// part 2
/////////////////////////////

function* allFrequencies(data, start){
  freq = start
  while (true) {
    for (const num of data){
      yield freq
      freq += num
    }
  }
}

const findRepeated = (frequencies) => {
  const seen = {}

  let repeat
  while(true){
    repeat = frequencies.next().value
    if (seen[repeat]) return repeat
    seen[repeat] = true
  }
  return repeat

}


fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  const dataList = processInput(data)

  const freqs = allFrequencies(dataList, 0)

  console.log('part 2:', findRepeated(freqs))

});