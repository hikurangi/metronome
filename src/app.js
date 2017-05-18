"use strict"

// const clickSound = document.querySelector('.click')
const clickSound = new Audio('audio/click.wav') // path relative to index.html, not this js file.
const onSwitch = document.querySelector('#onswitch')
// when the button is if the click is set to on in the state, start the click for an interval set by state.ms.
let clickInterval // create an ID for setInterval and clearInterval to dynamically change

const switchedOff = () => {
  clearInterval(clickInterval)
  state.currentBeat = 1
}

onSwitch.addEventListener('click', function() {
  // change button text
  state.switchedOn = !state.switchedOn
  this.innerHTML === "On" ? this.innerHTML = "Off" : this.innerHTML = "On"
  state.switchedOn ? clickInterval = setInterval(click, state.ms) : switchedOff()
})

const bpmInput = document.querySelector('#number-input')
bpmInput.addEventListener('input', function(e) {
  state.inputBPM = e.target.value // after changing these settings and turning the metch on + off again, the first two beats of each bar play, but not subsequent beats.
})

// clear input when clicked
bpmInput.addEventListener('focus', function() { // should be onClicks
  state.inputBPM = '' // does not clear values
})

// State / Config Object
const state = {
  inputBPM: 100, // set bpm from input. default to a leisurely 100bpm
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
  switchedOn: false, // metronome begins in off position
  clickArr: [] // perhaps should not be in state
}

// Sound Stacker - creates an array of cloned click sounds to allow the wav to overlap if necessary (fast tempos)
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
})

// The Click - actually plays the click sound
let click = () => {
  // log which beat we're at. this should be delayed by a quarter of the clickSound.duration so that it pops right about where the click itself is.
  console.log(`Beat #${state.currentBeat}`)
  // setTimeout(() => {}, clickSound.duration * 1000 / 4 ) // * 1000 miliseconds / 4 25% thru

  // play the click at the current position in the clickArr(ay)
  state.clickArr[state.arrPosition].play()

  // add one to the currentBeat tracker while it's less than the bar length, otherwise reset it to beat 1
  state.currentBeat < state.barLength ? state.currentBeat++ : state.currentBeat = 1
  // increment through the clickArray using the state's position tracker
  state.arrPosition < state.clickArr.length - 1 ? state.arrPosition++ : state.arrPosition = 0
}

// INITIAL GOALS

// trim wav down at front (not at tail)
// add tempo input
  // 1. when you turn on the metch, it blasts off at the tempo you specify
  // 2. event handler which updates the metronome tempo live (that might already work since the setInterval is using a tempo from state)
// add tap tempo
// css coloured bloop matches wave amplitude
// add subdivisions - 8ths, 16ths, 32nds, any tuplet
// slick interface

// STRETCH GOALS

// "musician features" - x bars on, x bars off
// x bars on - specify a percentage of clicks to drop out for x bars.
