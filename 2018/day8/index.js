const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

class Node {
  constructor(nChildren, nMetadata, children, metadata){
    this.nChildren = nChildren
    this.nMetadata = nMetadata
    this.children = children
    this.metadata = metadata
  }
}

const parseCode = data => data.trim().split(' ').map(num => parseInt(num))

const getNodes = (input, initialStart) => {
  let start = initialStart
  const nChildren = input[start]
  const nMetadata = input[start+1]

  start+=2

  const children = []
  let child
  for (let i=0; i<nChildren; i++){
    [child, start] = getNodes(input, start)
    children.push(child)
  }

  const metadata = input.slice(start, start+nMetadata)

  return [new Node(nChildren, nMetadata, children, metadata), start+nMetadata]
}

const sum = list => list.reduce((acc, val) => acc+val, 0)

const getSumMetadata = nodes => {
  const sumChild = nodes.nChildren
    ? nodes.children.reduce((acc, val) => acc + getSumMetadata(val), 0)
    : 0
    
  return sum(nodes.metadata) + sumChild
}


const ex=`2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2`

const sumMetadata = input => {
  const parsed = parseCode(input)
  const nodes = getNodes(parsed, 0)

  return getSumMetadata(nodes[0])
}

console.assert(sumMetadata(ex) == 138)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', sumMetadata(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const getValue = node => {
  if (node.nChildren == 0){
   return sum(node.metadata)
  } else {
    const childrenValues = {}
    for (let i=0; i<node.nChildren; i++){
      childrenValues[i] = getValue(node.children[i])  
    }
    let summed = 0
    for (let j=0; j<node.metadata.length; j++) {
      const idx = node.metadata[j]
      const toAdd = childrenValues[idx-1] || 0
      summed += toAdd
    }
    return summed
  }
}

const getValueOfNode = (input, idx) => {
  const parsed = parseCode(input)
  const nodes = getNodes(parsed, 0)
  const node = nodes[idx]
  return getValue(node)
}

console.assert(getValueOfNode(ex, 0) == 66)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', getValueOfNode(data, 0))
});

