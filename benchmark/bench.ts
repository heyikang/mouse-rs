import b from 'benny'

import mouse from '../index'

async function run() {
  await b.suite(
    'move tests',

    b.add('move to x: 100, y: 100', () => {
      mouse.moveTo(100, 100)
    }),

    b.add('move to x: 200, y: 200', () => {
      mouse.moveTo(200, 200)
    }),

    b.add('move to x: 300, y: 300', () => {
      mouse.moveTo(300, 300)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
