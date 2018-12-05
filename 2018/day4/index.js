const fs = require('fs')

const filename = process.argv[2];

/////////////////////////////
// part 1
/////////////////////////////

class Guard {
  constructor(id) {
    this.id = id
    this.actions = []
  }
  addAction(action) {
    this.actions.push(action)
  }
  getMinutesSlept() {
    const actions = this.actions
    return actions.reduce((acc, val, i) => {
      if (!/^falls/.test(val.comment) ){
        const sleepStart = actions[i-1].date.getMinutes()
        const sleepStop = val.date.getMinutes()
        const sleepTime = sleepStop - sleepStart
        const minutesSlept = [...new Array(sleepTime)].map((d,j) => j+sleepStart)
        acc = [...acc, ...minutesSlept]
      }
      return acc
    }, [])
  }
  getSleepTime(){
    const minutesSlept = this.getMinutesSlept()
    return minutesSlept.length
  }
  getPreferredMinuteToSleep(minuteOnly=true){
    const minutesSlept = this.getMinutesSlept().reduce((acc, val) => {
      acc[val] = ++acc[val] || 1
      return acc
    }, {})
    const topMinute = Object.entries(minutesSlept).reduce((acc, [min, val]) => {
      if (val > acc[0]) {
        acc[0] = val
        acc[1] = Number(min)
      }
      return acc
    }, [0, 0])
    if (minuteOnly) {
      return topMinute[1]
    } else {
      return topMinute
    }
  }
}

class Action {
  constructor(action) {
    const { date, comment } = action
    this.date = new Date(date)
    this.comment = comment
  }
}

const processInput = (data) => {
  // parse lines
  const regex = /\[(?<date>[^\]]+)\] (?<comment>[^]+)/
  return data.trim().split('\n').map(note => {
    const match = regex.exec(note)
    const { date, comment } = match.groups
    return new Action({ date, comment })
  })
}

const getGuardID = (action) => /Guard #(?<id>\d+)/.exec(action.comment).groups.id

const getGuardsSchedule = (sortedActions) => {
  const guardSchedule =  sortedActions.reduce((acc, action, i) => {
    // are we starting the actions of a new Guard?
    const isGuard = /^Guard/.test(action.comment)
    if (isGuard) {
      // create guard if not in database already
      const id = getGuardID(action)
      if (!acc[1][id]) acc[1][id] = new Guard(id)
      //update currentguard of accumulator
      acc[0] = id
    } else {
      // add action to current guard
      acc[1][acc[0]].addAction(action)
    }
    return acc 
  }, [0, {}])
  return Object.values(guardSchedule[1])
}

const getTopSleeper = (guards) => guards.reduce((top, guard) => {
  if (top == null) top = guard
  top = top.getSleepTime() > guard.getSleepTime()
    ? top
    : guard
  return top
}, null)


const ex=`[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up`

const analyzeData = (input) => {
  const sortedActions = processInput(input)
    .sort((a,b) => a.date-b.date) // sort chronologically
  const guards = getGuardsSchedule(sortedActions)
  const topSleeper = getTopSleeper(guards)
  const topMinute = topSleeper.getPreferredMinuteToSleep()
  return topSleeper.id*topMinute
}
console.assert(analyzeData(ex) == 240)




fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 1:', analyzeData(data))
});


/////////////////////////////
// part 2
/////////////////////////////

const analyzeData2 = (input) => {
  const sortedActions = processInput(input)
    .sort((a,b) => a.date-b.date) // sort chronologically
  const guards = getGuardsSchedule(sortedActions)
  const preferredMinutes = guards
    .map(guard => ([guard.id, guard.getPreferredMinuteToSleep(false)]))
    .sort((a,b) => b[1][0]-a[1][0])
  const [topId, [count, topMinute]] = preferredMinutes[0]
  return topId*topMinute
}

console.assert(analyzeData2(ex) == 4455)

fs.readFile(filename, 'utf8', (err, data) => {
  if (err) throw err;
  
  console.log('part 2:', analyzeData2(data))
});
