const { existsSync } = require('fs')
const { join } = require('path')

let nativeBinding = null
const binaryName = 'ctflags.node'

if (existsSync(join(__dirname, binaryName))) {
  nativeBinding = require(`./${binaryName}`)
} else {
  throw new Error(`Failed to load native binding: ${binaryName} not found`)
}

module.exports = nativeBinding
