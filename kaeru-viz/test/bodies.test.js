import assert from 'node:assert/strict'
import test from 'node:test'
import { memory, registerGraph } from '../src/api.js'
import { bodiesFor } from '../src/bodies.js'

test('lazy body cache belongs to the registered graph, including empty bodies', async () => {
  const original = memory.record
  let reads = 0
  memory.record = async () => { reads++; return reads === 1 ? '' : 'new body' }
  try {
    registerGraph({ snapshot_id: 'old', nodes: [{ id: 'a', initiatives: ['work'] }] })
    assert.deepEqual(await bodiesFor(['a']), { a: '' })
    assert.deepEqual(await bodiesFor(['a']), { a: '' })
    assert.equal(reads, 1)
    registerGraph({ snapshot_id: 'new', nodes: [{ id: 'a', initiatives: ['work'] }] })
    assert.deepEqual(await bodiesFor(['a']), { a: 'new body' })
    assert.equal(reads, 2)
  } finally { memory.record = original }
})

test('a pending read from an older graph cannot populate the new body cache', async () => {
  const original = memory.record
  let finish
  memory.record = () => new Promise((resolve) => { finish = resolve })
  try {
    registerGraph({ snapshot_id: 'pending-old', nodes: [{ id: 'a', initiatives: ['work'] }] })
    const pending = bodiesFor(['a'])
    registerGraph({ snapshot_id: 'pending-new', nodes: [{ id: 'a', initiatives: ['work'] }] })
    finish('old body')
    await assert.rejects(pending, { name: 'AbortError' })
    memory.record = async () => 'current body'
    assert.deepEqual(await bodiesFor(['a']), { a: 'current body' })
  } finally { memory.record = original }
})
