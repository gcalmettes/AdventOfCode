const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////


function* nextTurn(){
  const directions = ['left', 'straight', 'right']
  let i = 0
  while (true) {
    yield directions[i%3]
    i++
  }
}


class Cart {
  constructor(x, y, symbol) {
    this.x = x,
    this.y = y,
    this.direction = this.getDirection(symbol)
    this.nextTurn = nextTurn()
  }
  next(grid){
    // move
    if (this.direction == 'up') this.y-=1
    if (this.direction == 'down') this.y+=1
    if (this.direction == 'right') this.x+=1
    if (this.direction == 'left') this.x-=1
    // check if need to change direction
    const pos = `(${this.x},${this.y})`
    const track = grid[pos]
    if (track == '\\') {
      if (this.direction=='right') {
        this.direction = 'down'
      } else if (this.direction=='left') {
        this.direction = 'up'
      } else if (this.direction=='up') {
        this.direction = 'left'
      } else if (this.direction=='down') {
        this.direction = 'right'
      }
    }
    if (track == '/') {
      if (this.direction=='right') {
        this.direction = 'up'
      } else if (this.direction=='left') {
        this.direction = 'down'
      } else if (this.direction=='up') {
        this.direction = 'right'
      } else if (this.direction=='down') {
        this.direction = 'left'
      }
    }
    if (track == '+') {
      this.turn()
    }
    if (track == ' ') console.log('out of the track!!!!!')
  }
  turn(){
    const next = this.nextTurn.next().value
    if (next == 'left') {
      if (this.direction == 'right') {
        this.direction = 'up'
      } else if (this.direction == 'left') {
        this.direction = 'down'
      } else if (this.direction == 'down') {
        this.direction = 'right'
      } else if (this.direction == 'up') {
        this.direction = 'left'
      }
    } else if (next == 'right') {
      if (this.direction == 'right') {
        this.direction = 'down'
      } else if (this.direction == 'left') {
        this.direction = 'up'
      } else if (this.direction == 'down') {
        this.direction = 'left'
      } else if (this.direction == 'up') {
        this.direction = 'right'
      }
    }
  }
  getDirection(symbol) {
    if (symbol == '^') return 'up'
    if (symbol == 'v') return 'down'
    if (symbol == '>') return 'right'
    if (symbol == '<') return 'left'
  }
}


const processGrid = input => {
  const lines = input.split('\n')
  const carts = []
  const grid = lines.reduce((acc, line, y) => {
    // check if cart in line, if so get all matches
    const regex = /[v^><]/g
    while ((match = regex.exec(line)) != null) {
      carts.push(new Cart(match.index, y, match[0]))
    }
    // fill grid
    for (let x=0; x<line.length; x++) {
      const char = line[x]
      acc[`(${x},${y})`] = char
    }
    return acc
  }, {})
  // console.log('number of Carts:', carts.length)
  return [grid, carts]
}

const orderCarts = cartList => {
  // top carts have priority, then left most carts
  return cartList.sort((c1,c2) => c1.y - c2.y || c1.x - c2.x)
}

const checkCollisions = (cart, cartList) => {
  let collision = null
  cartList.forEach(d => {
    if (d != cart) {
      if (d.x == cart.x && d.y == cart.y) collision = {x: cart.x, y: cart.y}
    }
  })
  return collision
}

const simulate = input => {
  let [grid, carts] = processGrid(input)
  // console.log(grid)
  let collisions = []
  let stop = false
  let i = 0
  while (!stop) {
    i++
    // reorder carts
    carts = orderCarts(carts)
    carts.forEach(d => {
      d.next(grid)
      const isCol = checkCollisions(d, carts)
      if (isCol) {
        stop = true
        collisions.push(isCol)
      }
    })
  }
  return [i, collisions]
}

const ex = String.raw`/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   `

const [time, collision] = simulate(ex)
console.assert(`${collision[0].x},${collision[0].y}` == '7,3')



fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  const [time, collision] = simulate(data)
  console.log(`part 1: ${collision[0].x},${collision[0].y}`)

});



/////////////////////////////
// part 2
/////////////////////////////

const ex2 = String.raw`/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/`

const simulate2 = input => {
  let [grid, carts] = processGrid(input)
  let collisions = {}
  let stop = false
  let i = 0
  
  while (!stop) {
    i++
    // reorder carts
    carts = orderCarts(carts)
    const toRemove = []
    for (let c=0; c<carts.length; c++){
      const cart = carts[c]
      // check if cart was part of a collision in this tick
      if (collisions[i]){
        const collided = collisions[i].reduce((acc, col) => {
          return acc || (col.x == cart.x && col.y == cart.y)
        }, false)
        if (collided) continue
      } 
      cart.next(grid)
      const isCol = checkCollisions(cart, carts)
      if (isCol) {
        collisions[i] = collisions[i] || [] 
        collisions[i] = [...collisions[i], isCol]
        carts.forEach(c => {
          if ((c.x == isCol.x) && (c.y == isCol.y)) toRemove.push(c)
        })
      }
    }
    
    carts = carts.filter(c => !toRemove.includes(c))
    
    if (carts.length == 1) stop = true

  }
  // console.log(collisions)
  return carts[0]
}

const lastCart = simulate2(ex2)
console.assert(`${lastCart.x},${lastCart.y}` == '6,4')


fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  const lastCart = simulate2(data)
  console.log(`part 2: ${lastCart.x},${lastCart.y}`)

});
