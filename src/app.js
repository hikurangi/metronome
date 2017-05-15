"use strict"

let clickSound = document.querySelector('.click')

// config
let state = {
  inputBPM: 300, // set bpm from input
  tapTempoBPM: null, // set bpm from tap tempo
  get tapTempoMs() {
    return `no value yet ${this.tapTempoBPM}` // will use avg ms between taps. if setting tempo from this only start click from third tap?
  },
  get ms() {
    return 60000 / this.inputBPM
  },
  barLength: 4, // bar length in number of beats - default 4
  currentBeat: 1, // we start on (default to) beat 1 always
  arrPosition: 0, // track our position in the array of overlap sounds
  clickOn: true
}

clickSound.addEventListener("loadeddata", () => {

  // determine the minimum number of cloned sounds required.
  let loadLimit = Math.ceil(1000 * clickSound.duration / state.ms)

  let clickArr = []

  // push cloned sounds to a storage array
  for (let i = 0; i < loadLimit; i++) {
    // adding other subdivision/clicks will add a lot of complexity
    if ( clickArr.length < loadLimit ) {
      clickArr.push(clickSound.cloneNode())
    }
  }

  console.log({clickArr, state});
  // metronome click function - simplified solution works for slow tempos only. need to trim sound
  let click = () => {
    console.log(`Beat #${state.currentBeat}`)
    // check if a sound is already playing or the sound has not been played (this second option will be replaced with a check for whether )
    clickArr[state.arrPosition].play()

    // add one to it unless it is the same as the bar length
    state.currentBeat < state.barLength ? state.currentBeat++ : state.currentBeat = 1
    state.arrPosition < clickArr.length ? state.arrPosition++ : state.arrPosition = 0
  }

  if (state.clickOn) { setInterval(click, state.ms) }// may want to be a setTimeout within a for loop.

})
