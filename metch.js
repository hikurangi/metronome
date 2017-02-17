
// set bpm
let bpm = 300

// convert that bpm into miliseconds
let bpmInMs = 60000/bpm

// metronome click function
click = () => console.log("TICK!");

// print the word "TICK!" at that rate
setInterval(click, bpmInMs)
