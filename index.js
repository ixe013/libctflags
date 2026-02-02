const { existsSync } = require('fs')
const { join } = require('path')

let nativeBinding = null

if (existsSync(join(__dirname, 'index.node'))) {
  nativeBinding = require('./index.node')
} else {
  throw new Error('Failed to load native binding')
}

module.exports = nativeBinding
