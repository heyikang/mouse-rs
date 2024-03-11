import test from 'ava'

import mouse from '../index'

test('move to x: 100, y: 100', (t) => {
  t.is(mouse.moveTo(100, 100), true)
})

test('mouse get position', (t) => {
  t.deepEqual(mouse.getPosition(), { x: 100, y: 100 })
})
