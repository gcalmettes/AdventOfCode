const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

const processInput = (data) => {
  return data.trim().split('\n').map(line => {
    const { x, y } = /(?<x>\d+), (?<y>\d+)/.exec(line).groups
    return {x: +x, y: +y}
  })
}

const getExtremeCoords = coordList => {
  return coordList.reduce((canvas,coord) => {
    const { x1, x2, y1, y2 } = canvas
    return ({
      x1: coord.x<x1 ? coord.x : x1,
      x2: coord.x>x2 ? coord.x : x2,
      y1: coord.y<y1 ? coord.y : y1,
      y2: coord.y>y2 ? coord.y : y2,
    })
  }, {x1: Infinity, x2: -Infinity, y1: Infinity, y2: -Infinity})
}

const buildCanvas = coords => {
  const pixels = {}
  for (let i=coords.x1; i<coords.x2+1; i++) {
    for (let j=coords.y1; j<coords.y2+1; j++) {
      pixels[`(${i},${j})`] = {x: i, y: j}
    }
  }
  return pixels
}

const getDistance = (x,y,x2,y2) => {
  return (Math.abs(x2-x) + Math.abs(y2-y))
}

const getMaxArea = (data) => {
  const input = processInput(data)
  const extremes = getExtremeCoords(input)
  const pixels = buildCanvas(extremes)
  // calcul distance to each point
  Object.keys(pixels).forEach(pos => {
    const px = pixels[pos].x
    const py = pixels[pos].y
    pixels[pos]['distances'] = input.map(({x,y}) => {
      const c = `(${x},${y})`
      return {[c]: getDistance(px, py, x, y)}
    }).reduce((acc, val) => ({...acc, ...val}), {})
  })

  // count for each coordinates
  const count = Object.keys(pixels).reduce((count, p) => {
    const { distances } = pixels[p]
    const min = Object.keys(distances).reduce((acc, val) => distances[val] < distances[acc] ? val : acc, Object.keys(distances)[0])
    count[min] = ++count[min] || 1
    return count
  }, {})

  const largestArea = Object.values(count).reduce((acc, val) => val > acc ? val : acc, 0)
  return largestArea
}


const ex = `1, 1
1, 6
8, 3
3, 4
5, 5
8, 9`

console.assert(getMaxArea(ex) == 17)


fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', getMaxArea(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const getCompatibleUnitCount = (data, threshold=10000) => {
  const input = processInput(data)
  const extremes = getExtremeCoords(input)
  const pixels = buildCanvas(extremes)

  let count = 0

  // calcul distance to each point
  Object.keys(pixels).forEach(pos => {
    const px = pixels[pos].x
    const py = pixels[pos].y
    pixels[pos]['distances'] = input.map(({x,y}) => {
      const c = `(${x},${y})`
      return {[c]: getDistance(px, py, x, y)}
    }).reduce((acc, val) => ({...acc, ...val}), {})
    const isGood = Object.values(pixels[pos]['distances']).reduce((acc,val) => acc+val, 0) < threshold
    pixels[pos]['compatible'] = isGood
    if (isGood) count+=1
  })

  return count
}

console.assert(getCompatibleUnitCount(ex, 32) == 16)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', getCompatibleUnitCount(data, 10000))
});
