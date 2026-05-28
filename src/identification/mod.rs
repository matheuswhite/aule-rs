use crate::{math::float_point::FloatPoint, signal::Signal};

pub mod first_order;
pub mod second_order;

fn find_time_at_value<T>(signals: impl Iterator<Item = Signal<T>>, value: T) -> Option<T>
where
    T: FloatPoint,
{
    let mut closest_signal = None;
    let mut min_diff = T::infinity();

    for sig in signals {
        let diff = (sig.value - value).absolute();
        if diff < min_diff {
            min_diff = diff;
            closest_signal = Some(sig);
        }
    }

    Some(T::from_duration(closest_signal?.sim_state.sim_time()))
}
