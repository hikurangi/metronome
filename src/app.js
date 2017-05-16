"use strict"

const clickSound = document.querySelector('.click')

// config
const state = {
  inputBPM: 255, // set bpm from input
  tapTempoBPM: null, // set bpm from tap tempo
  get tapTempoMS() {
    return `no value yet ${this.tapTempoBPM}` // will use avg ms between taps. if setting tempo from this only start click from third tap?
  },
  get ms() {
    return 60000 / this.inputBPM
  },
  barLength: 4, // bar length in number of beats - default 4
  currentBeat: 1, // we start on (default to) beat 1 always
  arrPosition: 0, // track our position in the array of overlap sounds
  clickOn: true,
  clickArr: []
}

clickSound.addEventListener("loadeddata", () => {

  let loadLimit = Math.ceil(1000 * clickSound.duration / state.ms) // determine the minimum number of cloned sounds required. should change every time the tempo is changed

  for (let i = 0; i < loadLimit; i++) { // loop which will push cloned sounds to a storage array
    // adding other subdivision/clicks will add a lot of complexity
    if ( state.clickArr.length > loadLimit ) { // if the array of stored clicks is bigger than the necessary click limit
      state.clickArr = [] // reset the array
      state.clickArr.push(clickSound) // push the original click sound to it
    } else if ( state.clickArr.length < loadLimit ) { // otherwise if the clickArray is shorter than the click limit
      state.clickArr.push(clickSound.cloneNode()) // add another clone of the clickSound to it
    }

  }

  console.log('state.clickArr', state.clickArr, {state});

  // metronome click function

  let click = () => {
    // log which beat we're at. this should be delayed by half of the clickSound.duration so that it pops right about where the click itself is.
    setTimeout(() => {console.log(`Beat #${state.currentBeat}`)}, clickSound.duration * 500) // * 1000 miliseconds / 2 halfway thru

    // play the click at the current position in the clickArr(ay)
    console.log('state.clickArr', state.clickArr, {state});
    state.clickArr[state.arrPosition].play()

    // add one to the currentBeat tracker while it's less than the bar length, otherwise reset it to beat 1
    state.currentBeat < state.barLength ? state.currentBeat++ : state.currentBeat = 1
    // increment through the clickArray using the state's position tracker
    state.arrPosition < state.clickArr.length - 1 ? state.arrPosition++ : state.arrPosition = 0
  }

  // if the click is set to on in the state, start the click for an interval set by state.ms.
  if (state.clickOn) { setInterval(click, state.ms) } // may want to be a setTimeout within a for loop.

})

// add tap tempo
