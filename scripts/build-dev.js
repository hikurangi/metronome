const fs = require('fs-extra')
const path = require('path')

const buildDevDir = 'build-dev'

// Not sure how to get the fall-through to work with the switch
// switch(fs.existsSync(buildDevDir)) {
//   case true:
//     fs.remove(buildDevDir)
//   default:
//     fs.mkdirSync(buildDevDir)
// }


// 1. Create the build-dev directory. Delete and create if it exists
const buildSequence = new Promise ((resolve, reject) => {
  if (!fs.existsSync(buildDevDir)) {
    fs.mkdirSync(buildDevDir)
  } else {
    fs.removeSync(buildDevDir)
    fs.mkdirSync(buildDevDir)
  }
})
  .then(fs.copy('./public', './build-dev')) // 2. copy index.html, favicon.ico, manifest.json from ../public/ to build-dev/
  .then(() => {
    console.log('success!');
  })
  .catch(err => {
    console.log({err});
  })

// fs.createReadStream('test.log').pipe(fs.createWriteStream('newLog.log'));

// 3/2.5 change %PUBLIC_URL% in links to appropriate url in moved index.html
