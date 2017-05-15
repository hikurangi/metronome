"use strict"
  let clickSound = new Audio('audio/click.wav')
  const beatOne = document.querySelector('.beat-one')

  let config = {
    inputBPM: 100, // set bpm from input
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
  // is immediately invoked, resulting in NaN... what's up with that?

  let durationCheck = () => {
      return clickSound.duration;
  }
  clickSound.onloadeddata = durationCheck

  // how many sounds need to be preloaded in order to have enough?

  // convert durationCheck result into ms
  let soundDurationInMS = 1000 * clickSound.onloadeddata()
  console.log({soundDurationInMS});

  // needs to be done every time the tempo is reset

  // check whether the space between sounds in ms is

  // metronome click function
  let click = () => {
    // check if a sound is already playing or the sound has not been played (this second option will be replaced with a check for whether )
    let soundDurationInMS = 1000 * clickSound.onloadeddata()
    console.log({soundDurationInMS});

    if ( config.clickOn || clickSound.ended || clickCopy.ended ) {
      clickSound.play()
      console.log(`Beat #${config.currentBeat}`)
    } else {
      const clickCopy = clickSound.cloneNode()
      clickCopy.play()
      console.log(`Beat #${config.currentBeat}`)
    }

    // add one to it unless it is the same as the bar length
    config.currentBeat < config.barLength ? config.currentBeat++ : config.currentBeat = 1
  }

  // print the word "TICK!" at that rate
  setInterval(click, config.ms) // may want to be a setTimeout within a for loop.

  // click tempo - done
  // CLAP tempo using the machine's mic.
  // start stop
