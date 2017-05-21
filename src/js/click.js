import state from './state'

// The Click - actually plays the click sound
const click = () => {
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

module.exports = click
