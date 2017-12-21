`
http://adventofcode.com/2017/day/20
`

const data = require("./day20_input.js")

function getParticles(input) {
    return input.split("\n")
            .map(line => {
                const [, px, py, pz, vx, vy, vz ,ax, ay, az] = line.match(
                    /p=<([\d-]+),([\d-]+),([\d-]+)>, v=<([\d-]+),([\d-]+),([\d-]+)>, a=<([\d-]+),([\d-]+),([\d-]+)>/
                  ).map(Number)
                return {p: [px, py, pz], v: [vx, vy, vz], a: [ax, ay, az]}
                }
            )
}

function getSmallestAcc(particles){
    return particles.reduce(([minAcc, minIdx], {a: [ax, ay, az]}, i) => {
        let acc = Math.abs(ax) + Math.abs(ay) + Math.abs(az)
        return acc < minAcc ? [acc, i] : [minAcc, minIdx]
    }, [Infinity, 0])
}

function tick(particle) {
    const v = particle.v.map((d, i) => d + particle.a[i])
    return {
        p: particle.p.map((d, i) => d + v[i]),
        v: v,
        a: particle.a
        }
}

function filterOutCollided(particles){
    let countMap = particles.reduce((dict, d) => {dict[d.p] = (dict[d.p] || 0) + 1; return dict}, {})
    return particles.filter(d => countMap[d.p]==1)
}

function runSimulation(particlesList, nStable = 20) {
    let particles = particlesList.map(p => p)
    const state = {
                n: Infinity,
                iterations_since_stable: 0
            }

    while (state.iterations_since_stable < nStable) {
        particles = filterOutCollided(particles.map(particle => tick(particle)))
        if (state.n == particles.length) {
            state.iterations_since_stable +=1
        } else {
            state.n = particles.length
        }
    }
    return state.n
}



let particles = getParticles(data.input)
// part 1
console.log(`Index of particle with smallest acceleration: ${getSmallestAcc(particles)[1]}`)

// part 2
console.log(`Number of particles that didn't collide: ${runSimulation(particles)}`)
