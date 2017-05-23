# metronome

// BLOCKS
// 1. sound array does not fill completely after tempo is changed. need to look at control flow
// 2. webpack is fairly nightmarish

// INITIAL GOALS
// add tempo input
  // 1. when you turn on the metch, it blasts off at the tempo you specify
  // 2. event handler which updates the metronome tempo live (that might already work since the setInterval is using a tempo from state)
// add tap tempo
// css coloured bloop matches wave amplitude
// add subdivisions - 8ths, 16ths, 32nds, any tuplet
// slick interface
// implement tests with ava
//

// STRETCH GOALS
// "musician features" - x bars on, x bars off
// x bars on - specify a percentage of clicks to drop out for x bars.

// TESTING
// 1. when metch is switched on, it always starts with beat 1 - requires multiple runs ... actually difficult to test
//
