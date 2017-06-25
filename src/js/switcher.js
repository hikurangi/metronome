const switcher = (state, click, clickInterval) => {
  state.switched.on ? clickInterval = setInterval(click, state.ms) : ( clearInterval(clickInterval),
    state.currentBeat = 1)
}

export default switcher
