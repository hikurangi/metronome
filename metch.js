"use strict"

let config = {
  bpm: 120, // set bpm
  get ms() {
    return 60000 / this.bpm
  }, // does this getter work with dynamically shifting bpm?
  barLength: 4, // bar length - default 4/4
  currentBeat: 1 // we start on (default to) beat 1 always
}

// metronome click function
click = () => {

  // log the current beat number
  console.log(`Beat #${config.currentBeat}`)

  // add one to it unless it is the same as the bar length
  config.currentBeat < config.barLength ? config.currentBeat++ : config.currentBeat = 1

}

// print the word "TICK!" at that rate
setInterval(click, config.ms) // may want to be a setTimeout within a for loop.
