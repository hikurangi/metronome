const fs = require('fs-extra')
const path = require('path')

const buildDevDir = './build-dev'

if (fs.existsSync(buildDevDir)) {
  fs.mkdirSync(buildDevDir)
} else {
  fs.remove(buildDevDir)
  fs.mkdirSync(buildDevDir)
}

// 2. copy index.html, favicon.ico, manifest.json from ../public/ to build-dev/

// 3.
