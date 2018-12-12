const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

class Star {
  constructor(props){
    const { x, y, vx, vy } = props
    this.x = x
    this.y = y
    this.vx = vx
    this.vy = vy
  }
  calculatePositionAtTime(t){
    return {
      x: this.x + this.vx * t,
      y: this.y + this.vy * t
    }
  }
  getPositionAtTime(t){
    const pos = this.calculatePositionAtTime(t)
    return `(${pos.x},${pos.y})`
  }
}


class Canvas {
  constructor(stars){
    this.stars = stars
  }
  getBoundingBoxAtTime(t){
    return this.stars
      .map(star => star.calculatePositionAtTime(t))
      .reduce((box,star) => {
        box.xMin = star.x < box.xMin ? star.x : box.xMin
        box.xMax = star.x > box.xMax ? star.x : box.xMax
        box.yMin = star.y < box.yMin ? star.y : box.yMin
        box.yMax = star.y > box.yMax ? star.y : box.yMax
        return box
      }, {xMin: Infinity, xMax: -Infinity, yMin: Infinity, yMax: -Infinity})
  }
  computeAreaAtTime(t){
    const box = this.getBoundingBoxAtTime(t)
    return ((box.xMax-box.xMin)+1) * ((box.yMax-box.yMin)+1)
  }
  getTimeForMinArea(){
    let time = 0
    let prev = Infinity
    let area = this.computeAreaAtTime(time)
    while (area < prev) {
      prev = area
      area = this.computeAreaAtTime(++time)
    }
    return time-1
  }
  getStarPositionsAtTime(t){
    return this.stars
      .map(star => star.getPositionAtTime(t))
      .reduce((coordinates, star) => {
        coordinates[star] = true
        return coordinates
      }, {})
  }
  getStateAtTime(t){
    const box = this.getBoundingBoxAtTime(t)
    const stars = this.getStarPositionsAtTime(t)
    const pixels = {}
    for (let y=box.yMin; y<=box.yMax+1; y++){
      for (let x=box.xMin; x<=box.xMax; x++){
        const pos = `(${x},${y})`
        pixels[pos] = stars[pos] ? '#' : ' '
      }
    }
    return { box, pixels }
  }
  draw(t){
    const { box, pixels } = this.getStateAtTime(t)
    let lines = []
    let line = ' '
    let currentY = box.yMin
    Object.entries(pixels)
      .forEach(([pos, value]) => {
        const py = Number(/\((-?\d+),(-?\d+)\)/.exec(pos)[2])
        if (py == currentY) {
          line += value
        } else {
          lines.push(line)
          currentY = py
          line = ' ' + value
        }
      })
    console.log(lines.join('\n')+'\n')
  }
}

const getStars = input => {
  const regex = /position=<([-\s\d]\d+), ([-\s\d]\d+)> velocity=<([-\s\d]\d+), ([-\s\d]\d+)>/
  return input.trim().split('\n').map(line => {
    const [x, y, vx, vy] = regex.exec(line).slice(1, 5).map(Number)
    return  new Star({ x, y, vx, vy })
  })
}

ex = `position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>`


const getMessageAndTime = input => {
  const stars = getStars(input)
  const canvas = new Canvas(stars)
  const t = canvas.getTimeForMinArea()
  console.log(`====== ${t} seconds ======`)
  canvas.draw(t)
  return t
}

console.assert(getMessageAndTime(ex)==3)


fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  getMessageAndTime(data)
});

