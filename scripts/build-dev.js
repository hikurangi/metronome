const fs = require('fs-extra')
const path = require('path')

const buildDevDir = './build-dev'

// 1. Create the build-dev directory. Delete and create if it exists
if (!fs.existsSync(buildDevDir)) {
  fs.mkdirSync(buildDevDir)
} else {
  fs.remove(buildDevDir)
  fs.mkdirSync(buildDevDir)
}

// 2. copy index.html, favicon.ico, manifest.json from ../public/ to build-dev/

// 3.
