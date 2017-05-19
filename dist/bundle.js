/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// identity function for calling harmony imports with the correct context
/******/ 	__webpack_require__.i = function(value) { return value; };
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, {
/******/ 				configurable: false,
/******/ 				enumerable: true,
/******/ 				get: getter
/******/ 			});
/******/ 		}
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = 3);
/******/ })
/************************************************************************/
/******/ ([
/* 0 */
/***/ (function(module, exports) {

// removed by extract-text-webpack-plugin

/***/ }),
/* 1 */
/***/ (function(module, exports, __webpack_require__) {

module.exports = __webpack_require__.p + "d8ebb2aa988b75d743d7ee9b06593deb.wav";

/***/ }),
/* 2 */
/***/ (function(module, exports) {

module.exports = "<!doctype html>\r\n\r\n<html lang=\"en\">\r\n<head>\r\n  <meta charset=\"utf-8\">\r\n  <title>Metronome.js</title>\r\n  <meta name=\"description\" content=\"Metronomejs\">\r\n  <meta name=\"author\" content=\"Hikurangi\">\r\n  <link rel=\"stylesheet\" href=\"styles.css\">\r\n</head>\r\n\r\n<body>\r\n\r\n<input id=\"number-input\" type=\"number\">\r\n<button id=\"onswitch\" type=\"button\">Off</button> <!--- perhaps give the button no innerHTML and let the JS handle it --->\r\n<script src=\"bundle.js\"></script>\r\n\r\n</body>\r\n</html>\r\n";

/***/ }),
/* 3 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
Object.defineProperty(__webpack_exports__, "__esModule", { value: true });
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_0__css_styles_css__ = __webpack_require__(0);
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_0__css_styles_css___default = __webpack_require__.n(__WEBPACK_IMPORTED_MODULE_0__css_styles_css__);
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_1__audio_click_wav__ = __webpack_require__(1);
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_1__audio_click_wav___default = __webpack_require__.n(__WEBPACK_IMPORTED_MODULE_1__audio_click_wav__);
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_2__index_html__ = __webpack_require__(2);
/* harmony import */ var __WEBPACK_IMPORTED_MODULE_2__index_html___default = __webpack_require__.n(__WEBPACK_IMPORTED_MODULE_2__index_html__);


// Webpack imports




// const clickSound = document.querySelector('.click')
const clickSound = new Audio(__WEBPACK_IMPORTED_MODULE_1__audio_click_wav___default.a) // path relative to index.html, not this js file.
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
  if (state.switched.on) {
    clickInterval = setInterval(click, state.ms)
  } else {
    clearInterval(clickInterval)
    state.currentBeat = 1
  }
}

const bpmInput = document.querySelector('#number-input')
bpmInput.addEventListener('input', function(e) {
  state.inputBPM = e.target.value // after changing these settings and turning the metch on + off again, the first two beats of each bar play, but not subsequent beats.
  loader()
})

// clear input when clicked
bpmInput.addEventListener('focus', function() { // should be onClicks
  state.inputBPM = '' // does not clear values
})

// State / Config Object
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

// Sound Stacker - creates an array of cloned click sounds to allow the wav to overlap if necessary (fast tempos)
clickSound.addEventListener("loadeddata", () => {
  let loadLimit = Math.ceil(1000 * clickSound.duration / state.ms) // determine the minimum number of cloned sounds required. should change every time the tempo is changed
  loader()
})

// loadLimit calculates how many sounds to add to state.clickArray
const loadLimit = () => {
  console.log('loadLimit', Math.ceil(1000 * clickSound.duration / state.ms));
  return Math.ceil(1000 * clickSound.duration / state.ms)
}

// Sound loader - function to be invoked after loadLimit changes
const loader = () => {
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

// The Click - actually plays the click sound
let click = () => {
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

// BLOCKS
// 1. sound array does not fill completely after tempo is changed. need to look at control flow

// INITIAL GOALS
// add tempo input
  // 1. when you turn on the metch, it blasts off at the tempo you specify
  // 2. event handler which updates the metronome tempo live (that might already work since the setInterval is using a tempo from state)
// add tap tempo
// css coloured bloop matches wave amplitude
// add subdivisions - 8ths, 16ths, 32nds, any tuplet
// slick interface

// STRETCH GOALS
// "musician features" - x bars on, x bars off
// x bars on - specify a percentage of clicks to drop out for x bars.

// TESTING
// 1. when metch is switched on, it always starts with beat 1 - requires multiple runs ... actually difficult to test


/***/ })
/******/ ]);