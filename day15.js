function* generator(initial, factor, condition = (val) => true){
    let prev = initial;
    while (true) {
        let next = (prev*factor) % 2147483647;
        if (condition(next)) {
            yield next;
        };
        prev = next;
    };
};

function countOnes(nSample, genA, genB){
    let nOnes = 0;
    // only cares about last 16 bits
    const binaryMask = 0b00000000000000001111111111111111; 
    for (let i = 0; i<nSample ; i++){
        if ((genA.next().value & binaryMask) === (genB.next().value & binaryMask)) {
            nOnes += 1;
        };
    };
    return nOnes;
};

// part 1
const genA = generator(591, 16807)
const genB = generator(393, 48271)
console.log(`part 1: ${countOnes(40000000, genA, genB)}`)

// part 2
const genAp2 = generator(591, 16807, condition = (val) => val % 4 == 0)
const genBp2 = generator(393, 48271, condition = (val) => val % 8 == 0)
console.log(`part 2: ${countOnes(5000000, genAp2, genBp2)}`)
