use crate::engine::handle::EngineHandle;
use crate::session::handle::{Cmd, Phase, SessionHandle};
use std::sync::{Arc, atomic::Ordering};
use std::time::{Duration, Instant};

pub fn run(session: Arc<SessionHandle>, engine: Arc<EngineHandle>) {
    loop {
        let cmd = session.cmd.load(Ordering::Relaxed);
        if cmd == Cmd::Start as u8 {
            session.cmd.store(Cmd::None as u8, Ordering::Relaxed);
            match session.mode.load(Ordering::Relaxed) {
                1 => run_block(&session, &engine),
                2 => run_ladder(&session, &engine),
                _ => {}
            }
            // reset engine state after session ends
            engine.running.store(false, Ordering::Relaxed);
            session
                .phase
                .store(Phase::Finished as u8, Ordering::Relaxed);
            session.paused.store(false, Ordering::Relaxed);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Blocks until a bar boundary, checking for stop/pause commands.
/// Returns false if Stop was received.
fn wait_for_bar_boundary(session: &SessionHandle, engine: &EngineHandle) -> bool {
    engine.stop_at_bar.store(true, Ordering::Relaxed);
    let start_bar = engine.bar_count.load(Ordering::Relaxed);
    loop {
        if check_stop(session, engine) {
            return false;
        }
        if engine.bar_count.load(Ordering::Relaxed) > start_bar {
            return true;
        }
        std::thread::sleep(Duration::from_millis(8));
    }
}

/// Handles pause/resume, returns true if Stop was received.
fn check_stop(session: &SessionHandle, engine: &EngineHandle) -> bool {
    loop {
        let cmd = session.cmd.load(Ordering::Relaxed);
        if cmd == Cmd::Stop as u8 {
            session.cmd.store(Cmd::None as u8, Ordering::Relaxed);
            engine.running.store(false, Ordering::Relaxed);
            return true;
        }
        if cmd == Cmd::Pause as u8 {
            session.cmd.store(Cmd::None as u8, Ordering::Relaxed);
            session.paused.store(true, Ordering::Relaxed);
            engine.running.store(false, Ordering::Relaxed);
            // wait for resume or stop
            loop {
                let inner = session.cmd.load(Ordering::Relaxed);
                if inner == Cmd::Resume as u8 {
                    session.cmd.store(Cmd::None as u8, Ordering::Relaxed);
                    session.paused.store(false, Ordering::Relaxed);
                    engine.running.store(true, Ordering::Relaxed);
                    break;
                }
                if inner == Cmd::Stop as u8 {
                    session.cmd.store(Cmd::None as u8, Ordering::Relaxed);
                    engine.running.store(false, Ordering::Relaxed);
                    return true;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        return false;
    }
}

// ── Block ─────────────────────────────────────────────────────────────────────

fn run_block(session: &SessionHandle, engine: &EngineHandle) {
    let cfg = match session.block_config.read().unwrap().clone() {
        Some(c) => c,
        None => return,
    };

    let total_ms = cfg.duration.as_millis() as u64;
    session.phase.store(Phase::Playing as u8, Ordering::Relaxed);
    session.session_total_ms.store(total_ms, Ordering::Relaxed);
    session.session_elapsed.store(0, Ordering::Relaxed);

    engine.bpm.store(cfg.bpm, Ordering::Relaxed);
    engine.running.store(true, Ordering::Relaxed);

    let start = Instant::now();
    let mut paused_total = Duration::ZERO;
    let mut pause_start: Option<Instant> = None;

    loop {
        if check_stop(session, engine) {
            return;
        }

        // track paused time
        if session.paused.load(Ordering::Relaxed) {
            if pause_start.is_none() {
                pause_start = Some(Instant::now());
            }
        } else if let Some(ps) = pause_start.take() {
            paused_total += ps.elapsed();
        }

        let elapsed_ms = (start.elapsed() - paused_total).as_millis() as u64;
        session.session_elapsed.store(elapsed_ms, Ordering::Relaxed);
        // session
        //     .session_total_ms
        //     .store(total_ms.saturating_sub(elapsed_ms), Ordering::Relaxed);

        let remaining_ms = total_ms.saturating_sub(elapsed_ms);
        session.session_elapsed.store(elapsed_ms, Ordering::Relaxed);
        session
            .session_remaining_ms
            .store(remaining_ms, Ordering::Relaxed);

        if elapsed_ms >= total_ms {
            // wait for clean bar boundary before stopping
            if !wait_for_bar_boundary(session, engine) {
                return;
            }
            engine.running.store(false, Ordering::Relaxed);
            return;
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}

// ── Ladder ────────────────────────────────────────────────────────────────────

fn run_ladder(session: &SessionHandle, engine: &EngineHandle) {
    let cfg = match session.ladder_config.read().unwrap().clone() {
        Some(c) => c,
        None => return,
    };

    let total_ms = total_ladder_duration_ms(&cfg);
    session.session_total_ms.store(total_ms, Ordering::Relaxed);
    session
        .total_steps
        .store(cfg.cycle_count, Ordering::Relaxed);

    let mut current_bpm = cfg.start_bpm as i64;
    let mut session_elapsed_ms = 0u64;
    let session_start = Instant::now();

    for cycle in 0..cfg.cycle_count {
        session.current_step.store(cycle, Ordering::Relaxed);

        let bpm = current_bpm.max(1) as u64;
        let step_ms = cfg.step_duration.as_millis() as u64;
        let rest_ms = cfg.rest_duration.as_millis() as u64;

        // count-in: 4 beats at new tempo
        let beat_interval_ms = 60_000 / bpm;
        let count_in_ms = 4 * beat_interval_ms;

        // ── Rest phase ─────────────────────────────────────────────────────
        if rest_ms > 0 && cycle > 0 {
            engine.running.store(false, Ordering::Relaxed);
            session.phase.store(Phase::Resting as u8, Ordering::Relaxed);

            let rest_start = Instant::now();
            let effective_rest_ms = if rest_ms > count_in_ms && cycle > 0 {
                // leave room for count-in
                rest_ms - count_in_ms
            } else {
                rest_ms
            };

            loop {
                if check_stop(session, engine) {
                    return;
                }
                let elapsed = rest_start.elapsed().as_millis() as u64;
                if elapsed >= effective_rest_ms {
                    break;
                }
                update_session_timer(session, session_start, total_ms);
                std::thread::sleep(Duration::from_millis(16));
            }

            // ── Count-in ───────────────────────────────────────────────────
            if rest_ms > count_in_ms && cycle > 0 {
                session.phase.store(Phase::CountIn as u8, Ordering::Relaxed);
                engine.bpm.store(bpm, Ordering::Relaxed);
                engine.running.store(true, Ordering::Relaxed);

                let count_start = Instant::now();
                loop {
                    if check_stop(session, engine) {
                        return;
                    }
                    if count_start.elapsed().as_millis() as u64 >= count_in_ms {
                        break;
                    }
                    update_session_timer(session, session_start, total_ms);
                    std::thread::sleep(Duration::from_millis(16));
                }
                engine.running.store(false, Ordering::Relaxed);
            }
        }

        // ── Step phase ─────────────────────────────────────────────────────
        session.phase.store(Phase::Playing as u8, Ordering::Relaxed);
        session.step_total_ms.store(step_ms, Ordering::Relaxed);
        engine.bpm.store(bpm, Ordering::Relaxed);
        engine.running.store(true, Ordering::Relaxed);

        let step_start = Instant::now();
        loop {
            if check_stop(session, engine) {
                return;
            }
            let step_elapsed = step_start.elapsed().as_millis() as u64;
            session
                .step_elapsed_ms
                .store(step_elapsed, Ordering::Relaxed);
            update_session_timer(session, session_start, total_ms);

            if step_elapsed >= step_ms {
                if !wait_for_bar_boundary(session, engine) {
                    return;
                }
                engine.running.store(false, Ordering::Relaxed);
                break;
            }
            std::thread::sleep(Duration::from_millis(16));
        }

        current_bpm += cfg.tempo_increment;
    }
}

fn update_session_timer(session: &SessionHandle, start: Instant, total_ms: u64) {
    let elapsed = start.elapsed().as_millis() as u64;
    session.session_elapsed.store(elapsed, Ordering::Relaxed);
    session
        .session_total_ms
        .store(total_ms.saturating_sub(elapsed), Ordering::Relaxed);
}

fn total_ladder_duration_ms(cfg: &crate::session::config::LadderConfig) -> u64 {
    let step_ms = cfg.step_duration.as_millis() as u64;
    let rest_ms = cfg.rest_duration.as_millis() as u64;
    cfg.cycle_count as u64 * (step_ms + rest_ms)
}
