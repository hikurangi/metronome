"use strict"

let clickSound = document.querySelector('.click')

let config = {
  inputBPM: 400, // set bpm from input
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

  let loadLimit = Math.ceil(1000 * clickSound.duration / config.ms)

  let clickArr = []
  for (let i = 0; i < loadLimit; i++) {
    clickArr = []
    clickArr.push(clickSound.cloneNode())
  }

  console.log({clickArr, config});
  // metronome click function - simplified solution works for slow tempos only. need to trim sound
  let click = () => {
    clickArr.push(clickSound.cloneNode())
    console.log(`Beat #${config.currentBeat}`)
    // check if a sound is already playing or the sound has not been played (this second option will be replaced with a check for whether )
    clickArr[config.arrPosition].play()

    // add one to it unless it is the same as the bar length
    config.currentBeat < config.barLength ? config.currentBeat++ : config.currentBeat = 1

    config.arrPosition < clickArr.length ? config.arrPosition++ : config.arrPosition = 0
  }

  if (config.clickOn) { setInterval(click, config.ms) }// may want to be a setTimeout within a for loop.

})
