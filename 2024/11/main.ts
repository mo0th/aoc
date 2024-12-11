const input = await Bun.file('./input').text()

const stones = input.split(' ').map(s => parseInt(s))

const cache: Record<number, Record<number, number>> = {}

const results = stones.map(s => count(s, 75))

console.log(results)
const sum = results.reduce((a, b) => a + b, 0)
console.log(sum)

function count(
  stone: number,
  maxDepth: number,
  depth = 0,
  path: [number, number][] = []
): number {
  if (depth == maxDepth) {
    return 1
  }

  if (cache[stone]?.[depth]) {
    return cache[stone][depth]
  }

  const next = nextStones(stone)

  const nextPath: typeof path = [...path, [depth, stone]]

  const result = next
    .map(s => count(s, maxDepth, depth + 1, nextPath))
    .reduce((a, b) => a + b, 0)

  for (const [depth, stone] of nextPath) {
    cache[stone] ??= []
    cache[stone][depth] = result
  }

  return result
}

function nextStones(stone: number): number[] {
  if (stone === 0) {
    return [1]
  } else if (stone.toString().length % 2 === 0) {
    const left = stone.toString().slice(0, stone.toString().length / 2)
    const right = stone.toString().slice(stone.toString().length / 2)
    return [parseInt(left), parseInt(right)]
  } else {
    return [stone * 2024]
  }
}
