const pipe = (fn,...fns) => (...args) => fns.reduce( (acc, fn) => fn(acc), fn(...args));

const getDigits = recipeNumber => {
	const digits = []
	for (const d of String(recipeNumber)){
		digits.push(+d)
	}
	return digits
}

const combineRecipes =  (r1, r2) => r1+r2

const getNewRecipesFrom = (r1, r2) => pipe(
  combineRecipes,
  getDigits
)(r1, r2)

const moveTo = (pos, board) => {
  const size = board.length
  return (pos + 1 + board[pos]) % size
}

const getScoreboard = (r1, r2, nSteps) => {
  let board = [r1, r2]
  let pos1 = 0,
      pos2 = 1

  let s = 0
  while (s<nSteps) {
    // add new recipes to board
    board.push.apply(board, getNewRecipesFrom(board[pos1], board[pos2]))
    pos1 = moveTo(pos1, board)
    pos2 = moveTo(pos2, board)
    s++
    // stop if length is enough recipes on board
    if (board.length > nSteps+10) break
  }
  return board
}

const getRecipeScoresAfter = (r1, r2, nSteps, scoreOf = 10) => {
  const board = getScoreboard(r1, r2, nSteps+scoreOf)
  return board.slice(nSteps, nSteps+scoreOf).join('')
}

console.assert(getRecipeScoresAfter(3, 7, 5) == '0124515891')
console.assert(getRecipeScoresAfter(3, 7, 18) == '9251071085')
console.assert(getRecipeScoresAfter(3, 7, 2018) == '5941429882')

const INPUT = 554401

console.log('part1:', getRecipeScoresAfter(3, 7, INPUT))


const findInputInScoreBoard = (r1, r2, input) => {
  let board = [r1, r2]
  let pos1 = 0,
      pos2 = 1

  const inputSize = input.length
  let last, boardSize, nRecipes
  while (true) {
    // add recipes to board
    getNewRecipesFrom(board[pos1], board[pos2])
      .forEach(r => {
        board.push(r)
        // if we have correct combination, stop
        boardSize = board.length
        last = board.slice(boardSize-inputSize, boardSize).join('')
        if (last == input) nRecipes = boardSize-inputSize
      })
    if (nRecipes) break
    pos1 = moveTo(pos1, board)
    pos2 = moveTo(pos2, board)
  }
  return nRecipes
}

console.assert(findInputInScoreBoard(3, 7, '51589') == 9)
console.assert(findInputInScoreBoard(3, 7, '01245') == 5)
console.assert(findInputInScoreBoard(3, 7, '92510') == 18)
console.assert(findInputInScoreBoard(3, 7, '59414') == 2018)

console.log('part2:', findInputInScoreBoard(3, 7, String(INPUT)))
