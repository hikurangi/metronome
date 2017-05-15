"use strict"

  let clickSound = document.querySelector('.click')

  let config = {
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
    clickOn: true
  }

  // I want to play the same sound over and over and let it overlap as many times as necessary

  // app needs to calculate the maximum number of times this same sound will play simultaneously.

  // duration checker - needs to fire every time the clickOn is triggered OR
  // let loadLimit = null
  // clickSound.addEventListener("loadeddata", () => {
  //   let clickDuration = 1000 * clickSound.duration
  //   console.log("Audio duration in ms: ", clickDuration);
  //   console.log('config.ms', config.ms);
  //   loadLimit = Math.ceil(clickDuration / config.ms)
  //   console.log({loadLimit});
  // })

  // how many sounds need to be preloaded in order to have enough?
  // loadLimit is the number of sounds required


  // needs to be done every time the tempo is reset

  // check whether the space between sounds in ms is

  // metronome click function - simplified solution works for slow tempos only. need to trim sound
  let click = () => {

    console.log(`Beat #${config.currentBeat}`)
    // check if a sound is already playing or the sound has not been played (this second option will be replaced with a check for whether )
    clickSound.currentTime = 0
    clickSound.play()
    // add one to it unless it is the same as the bar length
    config.currentBeat < config.barLength ? config.currentBeat++ : config.currentBeat = 1

  }

  // print the word "TICK!" at that rate
  clickSound.addEventListener("loadeddata", () => {
    if (config.clickOn) { setInterval(click, config.ms) }// may want to be a setTimeout within a for loop.
  })
  // click tempo - done
  // CLAP tempo using the machine's mic.
  // start stop
