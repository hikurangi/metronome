
// set bpm
let bpm = 120

// convert that bpm into miliseconds
let ms = 60000/bpm

// bar length - default
let barLength = 4
let currentBeat = 1

// metronome click function
click = () => {
  // log the current beat number
  console.log(`Beat #${currentBeat}`)

  // add one to it unless it is the same as the bar length
  currentBeat < barLength ? currentBeat++ : currentBeat = 1
}

// print the word "TICK!" at that rate
setInterval(click, ms)
