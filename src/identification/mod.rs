use crate::signal::Signal;

pub mod first_order;
pub mod second_order;

fn find_time_at_value(signals: impl Iterator<Item = Signal<f64>>, value: f64) -> Option<f64> {
    let mut closest_signal = None;
    let mut min_diff = f64::INFINITY;

    for sig in signals {
        let diff = (sig.value - value).abs();
        if diff < min_diff {
            min_diff = diff;
            closest_signal = Some(sig);
        }
    }

    Some(closest_signal?.sim_state.sim_time().as_secs_f64())
}
