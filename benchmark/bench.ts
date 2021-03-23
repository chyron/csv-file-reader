import b from 'benny'
import path from 'path'

import { parse } from '../index'

const csvPath = path.join(__dirname, '../__test__/fixtures/basic.csv') 

async function run() {
  await b.suite(
    'parse basic csv',

    b.add('native', () => {
      parse(csvPath)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
