// loadLimit calculates how many sounds to add to state.clickArray
let loadLimit = (clickSound, state) => {
  console.log('loadLimit', Math.ceil(1000 * clickSound.duration / state.ms));
  return Math.ceil(1000 * clickSound.duration / state.ms)
}

export default loadLimit
