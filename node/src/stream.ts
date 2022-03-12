import { Readable } from 'stream';

const ITER_COUNT = 7;

async function* makeIter(n: number, fn: (i: number) => any) {
  for (let i = 0; i < n; i++) {
    console.log('gen', i)
    yield fn(i)
  }
}

async function logObjectStream(s: Readable) {
  for await (const x of s) {
    console.log('read', x)
  }
}

async function logBufferStream(s: Readable) {
  for await (const x of s) {
    console.log('read', x.toString())
  }
}

async function naive() {
  console.log('NAIVE')
  const s = Readable.from(makeIter(ITER_COUNT, (i) => ({k: i})))
  await logObjectStream(s)
}

async function normal() {
  console.log('NORMAL')
  const s = Readable.from(
    makeIter(ITER_COUNT, (i) => JSON.stringify({k: i})),
    {
      // highWaterMark counts bytes
      highWaterMark: 20,
      objectMode: false,
    }
  );
  await logBufferStream(s)
}

async function object() {
  console.log('OBJECT')
  const s = Readable.from(
    makeIter(ITER_COUNT, (i) => ({k: i})),
    {
      // highWaterMark counts objects
      highWaterMark: 3,
      objectMode: true,
    }
  );
  await logObjectStream(s)
}

// Wrapping an object stream leads to:
// TypeError [ERR_INVALID_ARG_TYPE]: The "chunk" argument must be of type string or an instance of Buffer or Uint8Array. Received an instance of Object
async function wrap() {
  console.log('WRAP')
  const s = Readable.from(
    makeIter(ITER_COUNT, (i) => JSON.stringify({k: i})),
    {
      objectMode: false
    }
  )
  const s2 = new Readable(
    {
      // highWaterMark counts bytes
      highWaterMark: 20,
      objectMode: false
    }
  ).wrap(s)
  await logBufferStream(s2)
}

async function main() {
  await naive()
  console.log()

  await normal()
  console.log()

  await object()
  console.log()

  await wrap()
  console.log()
}

main()
