import path from 'path'

import test from 'ava'

import { parse } from '../index'

test('parse simple csv', (t) => {
  const parsedCSV = parse(path.join(__dirname, 'fixtures', 'basic.csv'))
  t.snapshot(parsedCSV[0], 'first row')
  t.is(parsedCSV.length, 1, '1 row')
})

test('csv parse length equals 2 with no header', (t) => {
  const parsedCSV = parse(path.join(__dirname, 'fixtures', 'basic.csv'), { header: false })
  t.snapshot(parsedCSV[0], 'first row')
  t.snapshot(parsedCSV[1], 'second row')
  t.is(parsedCSV.length, 2, '2 rows')
})

test('csv more columns than headers', (t) => {
  const parsedCSV = parse(path.join(__dirname, 'fixtures', 'header.csv'))
  t.snapshot(parsedCSV[0], 'first row')
  t.is(parsedCSV.length, 1, '1 row')
})
