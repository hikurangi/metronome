"use strict"
  const clickSound = document.querySelector('.click')
  const beatOne = document.querySelector('.beat-one')

  let config = {
    inputBPM: 120, // set bpm from input
    tapTempoBPM: null, // set bpm from tap tempo
    get tapTempoMs() {
      return `no value yet ${this.tapTempoBPM}` // will use avg ms between taps. if setting tempo from this only start click from third tap?
    }
    get ms() {
      return 60000 / this.inputBPM
    },
    barLength: 4, // bar length in number of beats - default 4
    currentBeat: 1 // we start on (default to) beat 1 always
  }

  // metronome click function
  let click = () => {

    clickSound.play()
    // log the current beat number
    console.log(`Beat #${config.currentBeat}`)

    if (clickSound.ended || clickSound.currentTime === 0) {
          clickSound.play()
       } else {
          const clickCopy = clickSound.cloneNode()
          clickCopy.play()
       }

    // add one to it unless it is the same as the bar length
    config.currentBeat < config.barLength ? config.currentBeat++ : config.currentBeat = 1

  }

  // print the word "TICK!" at that rate
  setInterval(click, config.ms) // may want to be a setTimeout within a for loop.

  // click tempo
  // CLAP tempo using the machine's mic.
  // start stop
