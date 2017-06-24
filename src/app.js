"use strict"

// Webpack Imports
import webpackImports from './js/webpack-imports'

import clickWav from './audio/click.wav' // clickWav, click, clickSound very confusing

import { app } from './js/dom-setup'
console.log({app}); // make sure the destructuring is working

import click from './js/click'
import state from './js/state'
import loader from './js/loader'
import loadLimit from './js/loadLimit'

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
  loader(state, loadLimit, clickSound)
})

// clear input when clicked
bpmInput.addEventListener('focus', function() { // should be onClicks
  state.inputBPM = '' // does not clear values
})

// Sound Stacker - creates an array of cloned click sounds to allow the wav to overlap if necessary (fast tempos)
clickSound.addEventListener("loadeddata", () => {
  let loadLimit = Math.ceil(1000 * clickSound.duration / state.ms) // determine the minimum number of cloned sounds required. should change every time the tempo is changed
  loader(state, loadLimit, clickSound)
})
