// const input = await Deno.readTextFile('./sample')
const input = await Deno.readTextFile('./input')

type Packet = (number | Packet)[]
const pairs = input
  .split('\n\n')
  .map(line => line.split('\n').map(packet => eval(packet)) as [Packet, Packet])

const isNumber = (a: unknown): a is number => typeof a === 'number'
const isArray = Array.isArray

const packetValueToArrayIfNeeded = (value: Packet[number]): Packet =>
  Array.isArray(value) ? value : [value]

const compare = (left: Packet, right: Packet, depth = 1): -1 | 0 | 1 => {
  for (let i = 0; i < left.length; i++) {
    if (i >= right.length) {
      return -1
    }

    const leftItem = left[i]
    const rightItem = right[i]

    if (isNumber(leftItem) && isNumber(rightItem)) {
      if (leftItem < rightItem) {
        return 1
      }
      if (leftItem > rightItem) {
        return -1
      }
    } else if (isArray(leftItem) && isArray(rightItem)) {
      const tmp = compare(leftItem, rightItem, depth + 1)
      if (tmp !== 0) return tmp
    } else {
      const tmp = compare(
        packetValueToArrayIfNeeded(leftItem),
        packetValueToArrayIfNeeded(rightItem),
        depth + 1
      )
      if (tmp !== 0) return tmp
    }
  }

  if (left.length !== right.length) {
    return 1
  }
  return 0
}

const stringify = (value: Packet[number]): string =>
  Array.isArray(value) ? `[${value.map(v => stringify(v))}]` : value.toString()

const orderedIndices: number[] = []
pairs.forEach(([left, right], _i) => {
  const i = _i + 1
  const result = compare(left, right)
  if (result === 1) {
    orderedIndices.push(i)
  }
})

console.log(
  'A',
  orderedIndices.reduce((a, b) => a + b, 0)
)

const dividers: Packet[] = [[[2]], [[6]]]
const sorted = pairs
  .flatMap(p => p)
  .concat(dividers)
  .toSorted((a, b) => -compare(a, b))
console.log(
  'B',
  dividers.map(d => sorted.indexOf(d) + 1).reduce((a, b) => a * b, 1)
)
