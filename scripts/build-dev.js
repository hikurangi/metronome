const fs = require('fs-extra')
const path = require('path')

const buildDevDir = 'build-dev'

// 3/2.5 change %PUBLIC_URL% in links to appropriate url in moved index.html
const pathFix = () => {
  console.log('this is where index.html gets pointed to the correct url');
}

// 1. Create the build-dev directory. Delete and create if it exists
const buildSequence = new Promise ((resolve, reject) => {
  !fs.existsSync(buildDevDir) ? fs.mkdirSync(buildDevDir) : ( fs.removeSync(buildDevDir), fs.mkdirSync(buildDevDir) )
})
  .then(fs.copy('./public', './build-dev')) // 2. copy index.html, favicon.ico, manifest.json from ../public/ to build-dev/
  .then(pathFix())
  .then(() => {
    console.log('success!');
  })
  .catch(err => {
    console.log({err});
  })
