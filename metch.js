
// set bpm
let bpm = 120 //needs sensible limitations, especially with subdivisions

// convert that bpm into miliseconds
let ms = 60000/bpm

// bar length - default
let barLength = 4

// we start on (default to) beat 1 always
let currentBeat = 1

// metronome click function
click = () => {

  // log the current beat number
  console.log(`Beat #${currentBeat}`)

  // add one to it unless it is the same as the bar length
  currentBeat < barLength ? currentBeat++ : currentBeat = 1

}

// print the word "TICK!" at that rate
setInterval(click, ms) // may want to be a setTimeout within a for loop.
