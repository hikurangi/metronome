// App State
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
  switched: { // a single source of truth
    on: false, // metronome begins in off position
    get text() {
      return this.on === true ? "On" : "Off"
    }
  },
  clickArr: [] // perhaps should not be in state
}

module.exports = state
