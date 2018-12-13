const computePowerLevelOfCell = ({x, y}, serialNumber) => {
  const rackId = x + 10
  let powerLevel = ((rackId * y) + serialNumber) * rackId
  let digit = Math.floor(powerLevel/100) % 10
  powerLevel = digit - 5
  return powerLevel
}

console.assert(computePowerLevelOfCell({x: 3, y: 5}, 8) == 4)
console.assert(computePowerLevelOfCell({x: 122, y: 79}, 57) == -5)
console.assert(computePowerLevelOfCell({x: 217, y: 196}, 39) == 0)
console.assert(computePowerLevelOfCell({x: 101, y: 153}, 71) == 4)

const getGrid = (width, height) => {
  const grid = {}
  for (let y=1; y<height+1; y++){
    for (let x=1; x<width+1; x++){
      grid[`(${x},${y})`] = { x, y }
    }
  }
  return grid
}

const getSquare = ({x,y}, length=3) => {
  // topLeftCorner = {x,y}
  const allCells = []
  for (let j=0; j<length; j++){
    for (let i=0; i<length; i++){
      allCells.push({x: x+i, y: y+j})
    }
  }
  return allCells
}

const computePowerLevelOfSquare = ({x,y}, serialNumber, squareLength=3) => {
  const allCells = getSquare({x,y}, squareLength)
  return allCells.reduce((sum, cell) => {
    const cellPower = computePowerLevelOfCell(cell, serialNumber)
    return sum + cellPower
  }, 0)
}

console.assert(computePowerLevelOfSquare({x:33, y:45}, 18) == 29)
console.assert(computePowerLevelOfSquare({x:21, y:61}, 42) == 30)


const crawlGrid = (width, height, serialNumber, squareLength=3) => {
  let max = [-Infinity, {}]
  for (let y=1; y<(height+1)-squareLength; y++){
    for (let x=1; x<(width+1)-squareLength; x++){
      const powerLevel = computePowerLevelOfSquare({ x, y }, serialNumber, squareLength)
      if (powerLevel > max[0]) {
        max = [powerLevel, { x, y }]
      }
    }
  }
  return max
}

var [powerLevel, cell] = crawlGrid(300, 300, 18)
console.assert(powerLevel == 29)
console.assert(cell.x == 33)
console.assert(cell.y == 45)

var [powerLevel, cell] = crawlGrid(300, 300, 42)
console.assert(powerLevel == 30)
console.assert(cell.x == 21)
console.assert(cell.y == 61)

/////////////////////////////
// part 1
/////////////////////////////

const INPUT = 6042
const [p1Power, p1Cell] = crawlGrid(300, 300, INPUT)
console.log('part 1:', `${p1Cell.x},${p1Cell.y}`)

/////////////////////////////
// part 2
/////////////////////////////

// This is not the proper way to solve the problem, but it works ...
// Better way would be to implement a summed area table
// see https://en.wikipedia.org/wiki/Summed-area_table
// and https://www.codeproject.com/Articles/441226/Haar-feature-Object-Detection-in-Csharp


let max = [-Infinity, {}, 1]
for (let s=1; s<=300; s++){
  const [p2Power, p2Cell] = crawlGrid(300, 300, INPUT, s)
  //console.log(s, p2Power)
  if (p2Power > max[0]) {
    max = [p2Power, p2Cell, s]
  } else {
  // no need to check further
  // the idea is that the max power will have a global maximum.
  // Saw that when console logging the max power for each square sixze
    break
  }
}

console.log('part 2:', `${max[1].x},${max[1].y},${max[2]}`)


