const { parse } = require('./index')
const path = require('path')

const csvPath = path.join(__dirname, '__test__/fixtures/basic.csv')

console.info('testing basic.csv parsing')

console.assert(parse(csvPath).length == 1, 'Simple test failed')

console.info('Simple test passed')
