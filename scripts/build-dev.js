const fs = require('fs-extra')
const path = require('path')

const buildDevDir = 'build-dev'

// 1. Create the build-dev directory. Delete and create if it exists
const buildSequence = new Promise ((resolve, reject) => {
  !fs.existsSync(buildDevDir) ? fs.mkdirSync(buildDevDir) : ( fs.removeSync(buildDevDir), fs.mkdirSync(buildDevDir) )
}) // must happen before files are copied to it.
  .then(fs.copy('./public', './build-dev')) // 2. copy index.html, favicon.ico, manifest.json from ../public/ to build-dev/
  .then(pathFix()) // 3. see at function definition
  .then(() => {
    console.log('success!');
  })
  .catch(err => {
    console.log({err});
  })

// 3/2.5 change %PUBLIC_URL% in links to appropriate url in moved index.html
const pathFix = () => {
  console.log('this is where index.html gets pointed to the correct url');
}
