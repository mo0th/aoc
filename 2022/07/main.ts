const sample = Deno.readTextFileSync('./sample')
const input = Deno.readTextFileSync('./input')

type File = {
  name: string
  size: number
}

type Dir = {
  name: string
  files: File[]
  dirs: Dir[]
  size: number
  sized: boolean
}

const stack: Dir[] = [
  {
    name: '$$$root',
    dirs: [
      {
        name: '/',
        files: [],
        dirs: [],
        size: 0,
        sized: false,
      },
    ],
    files: [],
    size: 0,
    sized: false,
  },
]

input.split('\n').forEach(line => {
  if (line.startsWith('$')) {
    const [_, cmd, arg] = line.split(' ')
    if (cmd === 'cd') {
      if (arg === '..') {
        stack.pop()
      } else {
        const toPush = stack.at(-1)?.dirs.find(d => d.name === arg)
        if (toPush) stack.push(toPush)
      }
    }
  } else {
    const [typeOrSize, name] = line.split(' ')
    const last = stack.at(-1)
    if (typeOrSize === 'dir') {
      last?.dirs.push({
        name,
        files: [],
        dirs: [],
        size: 0,
        sized: false,
      })
    } else {
      last?.files.push({
        name,
        size: parseInt(typeOrSize),
      })
    }
  }
})

const small: number[] = []

const all: number[] = []

const setSizes = (dir: Dir) => {
  if (dir.sized) return

  dir.dirs.forEach(d => setSizes(d))

  dir.size = [...dir.dirs, ...dir.files]
    .map(f => f.size)
    .reduce((a, b) => a + b)

  if (dir.size < 100_000) {
    small.push(dir.size)
  }
  all.push(dir.size)

  dir.sized = true
}

setSizes(stack[0])

const unused = 70000000 - stack[0].size
const needed = 30000000 - unused

console.log(
  'a',
  small.reduce((a, c) => a + c)
)
console.log(
  'b',
  all.toSorted((a, b) => a - b).find(el => el > needed)
)
