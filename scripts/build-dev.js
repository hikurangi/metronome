const fs = require('fs-extra')
const path = require('path')
require('dotenv').config() // *may* be needed elsewhere
const jsdom = require('jsdom')
const buildDevDir = 'build-dev' // could abstract many of these things into .env

// 3/2.5 change %PUBLIC_URL% in links to appropriate url in moved index.html
const pathFix = () => {
  console.log('this is where index.html gets pointed to the correct url');
  console.log('process.env.NODE_ENV', process.env.NODE_ENV);
  let index = '../build-dev/index.html'
  // use __dirname and path.resolve to find index.html and bundle.js <---   // 1. move index
  // 2. trigger webpack (not dev server?) to move the file to the build folder
  // 3. after that's been done, do a path.resolve to find relative path from index.html to bundle.js and replace %PUBLIC_URL% with that path
  //
}

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
    console.error({err});
  })
