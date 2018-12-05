const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

const processInput = (data) => {
  // each claim is in the form #id @ x,y: width*height
  // return an object will all the characteristics
  const regex = /#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<width>\d+)x(?<height>\d+)/
  return data.trim().split('\n').map(claim => {
    const match = regex.exec(claim)
    const matchObject = Object.entries(match.groups)
      .reduce((acc, cur) => {
        acc[cur[0]] = Number(cur[1])
        return acc 
      }, {})
    return matchObject
  })
}

const getCanvas = (data) => {
  return data.reduce((acc, cur) => {
    // add claim to canvas
    for (let i=cur.x; i<cur.x+cur.width; i++){
      for (let j=cur.y; j<cur.y+cur.height; j++){
        acc[`(${i},${j})`] = ++acc[`(${i},${j})`] || 1
      }
    }
    return acc
  }, {})
}

const getOverlap = (data) => {
  const processed = processInput(data)
  const canvas = getCanvas(processed)
  const overlapCount = Object.values(canvas).filter(d => d>1)
  return overlapCount.length
}

const ex=`#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2`

console.assert(getOverlap(ex) == 4)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', getOverlap(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const getCanvasWithIds = (data) => {
  return data.reduce((acc, cur) => {
    // add claim to canvas and declare which IDs are on this square
    for (let i=cur.x; i<cur.x+cur.width; i++){
      for (let j=cur.y; j<cur.y+cur.height; j++){
        let coord = acc[`(${i},${j})`]
        let { count, ids } = { ...coord }
        ids = ids || []
        coord = {count: ++count || 1, ids: [...ids, cur.id]}
        acc[`(${i},${j})`] = coord
      }
    }
    return acc
  }, {})
}

const getNonOverlappingIds = (input) => {
  const processed = processInput(input)
  const canvasWithIds = getCanvasWithIds(processed)
  const ids = processed.map(d => d.id)
  let overlappingIds = Object.values(canvasWithIds).map(d => d.ids).filter(d => d.length>1)
  overlappingIds = overlappingIds.reduce((acc, cur) => new Set([...acc, ...cur]), new Set())
  return ids.map(id => [id, overlappingIds.has(id)]).filter(d => !d[1]).map(d => d[0])
}

console.assert(getNonOverlappingIds(ex)[0] == 3)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', getNonOverlappingIds(data))
});




