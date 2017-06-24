// Sound loader - function to be invoked after loadLimit changes
let loader = (state, loadLimit, clickSound) => {
  console.log({stateInLoader: state});

  let limit = 0

  typeof loadlimit === 'function' ? limit = loadLimit(state, clickSound) : limit = loadLimit
  for (let i = 0; i < loadLimit; i++) { // loop which will push cloned sounds to a storage array
    // adding other subdivision/clicks will add a lot of complexity
    if ( state.clickArr.length > limit ) { // if the array of stored clicks is bigger than the necessary click limit
      state.clickArr = [] // reset the array
      state.clickArr.push(clickSound) // push the original click sound to it
    } else if ( state.clickArr.length < limit ) { // otherwise if the clickArray is shorter than the click limit
      state.clickArr.push(clickSound.cloneNode()) // add another clone of the clickSound to it
    }
  }
}

export default loader
