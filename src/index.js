"use strict"

// Webpack imports
import styles from './css/index.css'
import clickWav from './audio/click.wav' // clickWav, click, clickSound very confusing

import { app } from './js/dom-setup'
console.log({app}); // make sure the destructuring is working

import click from './js/click'
import state from './js/state'
// const clickSound = document.querySelector('.click')
const clickSound = new Audio(clickWav)
const onSwitch = document.querySelector('#onswitch')
// when the button is if the click is set to on in the state, start the click for an interval set by state.ms.

// abstracted but very readable imo
onSwitch.addEventListener('click', function() {
  state.switched.on = !state.switched.on
  this.innerHTML = state.switched.text
  switcher()
})

let clickInterval // create an ID for setInterval and clearInterval to dynamically change
const switcher = () => {
  state.switched.on ? clickInterval = setInterval(click, state.ms) : ( clearInterval(clickInterval),
    state.currentBeat = 1)
}

let bpmInput = document.querySelector('#number-input')
bpmInput.addEventListener('input', function(e) {
  state.inputBPM = e.target.value // after changing these settings and turning the metch on + off again, the first two beats of each bar play, but not subsequent beats.
  loader()
})

// clear input when clicked
bpmInput.addEventListener('focus', function() { // should be onClicks
  state.inputBPM = '' // does not clear values
})

// Sound Stacker - creates an array of cloned click sounds to allow the wav to overlap if necessary (fast tempos)
clickSound.addEventListener("loadeddata", () => {
  let loadLimit = Math.ceil(1000 * clickSound.duration / state.ms) // determine the minimum number of cloned sounds required. should change every time the tempo is changed
  loader()
})

// loadLimit calculates how many sounds to add to state.clickArray
let loadLimit = () => {
  console.log('loadLimit', Math.ceil(1000 * clickSound.duration / state.ms));
  return Math.ceil(1000 * clickSound.duration / state.ms)
}

// Sound loader - function to be invoked after loadLimit changes
let loader = () => {
  console.log({state});
  loadLimit()
  for (let i = 0; i < loadLimit; i++) { // loop which will push cloned sounds to a storage array
    // adding other subdivision/clicks will add a lot of complexity
    if ( state.clickArr.length > loadLimit ) { // if the array of stored clicks is bigger than the necessary click limit
      state.clickArr = [] // reset the array
      state.clickArr.push(clickSound) // push the original click sound to it
    } else if ( state.clickArr.length < loadLimit ) { // otherwise if the clickArray is shorter than the click limit
      state.clickArr.push(clickSound.cloneNode()) // add another clone of the clickSound to it
    }
  }
}
