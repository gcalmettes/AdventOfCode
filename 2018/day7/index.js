const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

class Step {
  constructor(name, next) {
    this.name = name
    this.next = next
  }
}

const parseSteps = (data) => {
  return data.trim().split('\n').map(line => {
    const { step, next } = /Step (?<step>[A-Z]) must be finished before step (?<next>[A-Z]) can begin\./.exec(line).groups
    return new Step(step, next)
  })
}

const getGraph = stepsList => {
  return stepsList.reduce((graph, step) => {
    const { name, next } = step
    // add to graph if not exist, or modify
    const [stepExists, nextStepExists] =  [graph[name], graph[next]]
    // current step
    if (stepExists) {
      const { isRoot, childs } = graph[name]
      graph[name] = {
        isRoot: true && isRoot,
        childs: new Set([...childs, next])
      }
    } else {
      graph[name] = {
        isRoot: true,
        childs: [next]
      }
    }
    // next step
    if (nextStepExists) {
      const { isRoot, childs } = graph[next]
      graph[next] = {
        isRoot: false,
        childs: graph[next].childs
      }
    } else {
      graph[next] = {
        isRoot: false,
        childs: new Set()
      }
    }
    return graph
  }, {})
}


const getOrder = graph => {
  // all possible steps
  let remainingSteps = Object.keys(graph)
  // will hold the ordered steps
  let order = []

  // keep track of the potential next steps, starting with all the potential roots
  let stepsInLine = Object.keys(graph)
    .filter(node => graph[node].isRoot)
    .map(name => name)

  let current = true
  while (current) {
    // filter out steps that have been done
    remainingSteps = remainingSteps.filter(d => !order.includes(d))
    stepsInLine = stepsInLine.filter(d => !order.includes(d))
    
    // filter out steps in line that are childs of other steps in line
    const nextStepsAllowed = stepsInLine.reduce((acc, cur) => {
      const isValidStep = remainingSteps.reduce((a,s) => a && !graph[s].childs.has(cur), true)
      if (isValidStep) acc.push(cur)
      return [...new Set(acc)]
    }, [])
      .sort((a,b) => b>a)

    // add valid step to order
    current = nextStepsAllowed.pop()
    order.push(current)

    // add new childs to stepsInLine
    if (current) stepsInLine = [...stepsInLine, ...Array.from(graph[current].childs)]
      .reduce((acc, cur) => {
        acc = !acc.includes(cur) ? [...acc, cur] : acc
        return acc
      }, [])
  }
  return order.join('')
}

const ex=`Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.`


const getStepsOrder = (input) => {
  const parsedSteps = parseSteps(input)
  const graph = getGraph(parsedSteps)
  return getOrder(graph)
}

console.assert(getStepsOrder(ex) == 'CABDFE')


fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', getStepsOrder(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const getAlphabetPosition = char => 1 + (parseInt(char, 36) - 10)

const getTimeToCompleteSteps = (input, nWorkers, timeShift) => {
  const parsedSteps = parseSteps(input)
  const graph = getGraph(parsedSteps)

  // declare the workers
  const workers = [...new Array(nWorkers)].map(d => ({step: undefined, stepTimeLeft: 0}))

  // all possible steps
  let remainingSteps = Object.keys(graph)
  // will hold the ordered steps
  let order = []
  let stepsInProcess = []

  // keep track of the potential next steps, starting with all the potential roots
  let stepsInLine = Object.keys(graph)
    .filter(node => graph[node].isRoot)
    .map(name => name)

  let time = -1
  let running = true
  while (running) {
    // filter out steps that have been done
    remainingSteps = remainingSteps.filter(d => !order.includes(d))
    stepsInProcess = stepsInProcess.filter(d => !order.includes(d))
    stepsInLine = stepsInLine.filter(d => !order.includes(d) && !stepsInProcess.includes(d))
    
    // filter out steps in line that are childs of other steps in line
    const nextStepsAllowed = stepsInLine.reduce((acc, cur) => {
      const isValidStep = remainingSteps.reduce((a,s) => a && !graph[s].childs.has(cur), true)
      if (isValidStep) acc.push(cur)
      return [...new Set(acc)]
    }, [])
      .sort((a,b) => b>a)

    const finishedSteps = []
    // if workers are done with their step, give a new one
    for (let i=0; i<workers.length; i++){
      const worker = workers[i]
      if (worker.step) workers[i].stepTimeLeft -= 1
      
      if (worker.stepTimeLeft <= 0) {
        // add step to completed steps
        if (worker.step){
          order.push(worker.step)
          finishedSteps.push(worker.step)
        } 
        
        // get new step to do if any
        const toStart = nextStepsAllowed.pop()
        if (toStart) stepsInProcess.push(toStart)
        workers[i] = {step: toStart, stepTimeLeft: toStart ? getAlphabetPosition(toStart) + timeShift - 1 : 0}
      } 
    }

    // check if some workers have step in process
    running = workers.reduce((a,w) => w.step || a, false) || finishedSteps.length!=0

    // console.log(workers)

    const newChilds = finishedSteps.reduce((acc, s) => {
      acc = [...acc, ...[...graph[s].childs]]
      return acc
    }, [])

    // add new childs to stepsInLine
    if (running) stepsInLine = [...stepsInLine, ...newChilds]
      .reduce((acc, cur) => {
        acc = !acc.includes(cur) ? [...acc, cur] : acc
        return acc
      }, [])
    time+=1

  }
  return time

}


console.assert(getTimeToCompleteSteps(ex, 2, 0) == 15)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', getTimeToCompleteSteps(data, 5, 60))
});
 