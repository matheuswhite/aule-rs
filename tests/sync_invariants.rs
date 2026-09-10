//! Invariants of the sync blocks that the type system does not prove on its own.
//!
//! These are integration tests on purpose: they reach the blocks only through the
//! public API, so a failure here means *safe user code* can break the invariant.
//! That is the property worth guarding — an in-module test could reach internals
//! and prove less.
//!
//! Each test declares its own block. The handles are claimed once and never
//! released, and cargo runs tests as threads of a single process, so a shared block
//! would let whichever test ran first starve the others.
//!
//! The billboard tests drive `Block::block` directly instead of going through the
//! multiplication operator, so that a failure points at the slot protocol rather
//! than at the composition path.

use aule::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static CREATED: AtomicUsize = AtomicUsize::new(0);
static DROPPED: AtomicUsize = AtomicUsize::new(0);

/// Counts its own construction and destruction. Only the ownership test uses it,
/// so the counters are not shared across tests.
#[derive(Debug)]
struct Tracked {
    _payload: [u64; 2],
}

impl Tracked {
    fn new() -> Self {
        CREATED.fetch_add(1, Ordering::SeqCst);
        Self {
            _payload: [0xAA; 2],
        }
    }
}

impl Drop for Tracked {
    fn drop(&mut self) {
        DROPPED.fetch_add(1, Ordering::SeqCst);
    }
}

/// A value fed into the queue and picked back out must be destroyed exactly once.
///
/// The feeder copies the payload bytes into the queue and forgets its local; the
/// picker reclaims the value with `ptr::read`. If the feeder ever let its local
/// drop, the value would be destroyed twice — a double free for any payload that
/// owns memory.
#[test]
fn conveyor_transfers_ownership_exactly_once() {
    static CONVEYOR: Conveyor<Tracked, 8> = Conveyor::new();

    let mut sim = EndlessSimulation::new(1.0);
    let mut feeder = CONVEYOR.feeder().expect("the first feeder must be available");
    let mut picker = CONVEYOR.picker().expect("the first picker must be available");

    let _ = Tracked::new().as_signal(sim.next().unwrap()) * feeder.as_block();

    let picked = sim.next().unwrap() * picker.as_block();
    assert!(picked.value.is_ok(), "the picker found the queue empty");
    drop(picked);

    let created = CREATED.load(Ordering::SeqCst);
    let dropped = DROPPED.load(Ordering::SeqCst);
    assert_eq!(
        dropped, created,
        "{dropped} drops for {created} values created"
    );
}

/// A payload that owns heap memory must survive the round trip intact.
///
/// If the feeder dropped its local instead of forgetting it, the buffer would be
/// freed while the queue still held a bitwise copy of the pointer, and the picked
/// value would read freed memory.
#[test]
fn conveyor_moves_a_heap_owning_payload() {
    static CONVEYOR: Conveyor<String, 64> = Conveyor::new();

    let mut sim = EndlessSimulation::new(1.0);
    let mut feeder = CONVEYOR.feeder().expect("the first feeder must be available");
    let mut picker = CONVEYOR.picker().expect("the first picker must be available");

    let _ = String::from("setpoint-critico").as_signal(sim.next().unwrap()) * feeder.as_block();

    let picked = sim.next().unwrap() * picker.as_block();
    assert_eq!(
        picked.value.expect("the picker found the queue empty"),
        "setpoint-critico"
    );
}

/// The queue is single-producer, so the producer handle is handed out once.
#[test]
fn conveyor_yields_at_most_one_feeder() {
    static CONVEYOR: Conveyor<f32, 8> = Conveyor::new();

    assert!(
        CONVEYOR.feeder().is_some(),
        "the first feeder must be available"
    );
    assert!(
        CONVEYOR.feeder().is_none(),
        "a second feeder would give the queue two producers"
    );
}

/// The queue is single-consumer, so the consumer handle is handed out once.
#[test]
fn conveyor_yields_at_most_one_picker() {
    static CONVEYOR: Conveyor<f32, 8> = Conveyor::new();

    assert!(
        CONVEYOR.picker().is_some(),
        "the first picker must be available"
    );
    assert!(
        CONVEYOR.picker().is_none(),
        "a second picker would give the queue two consumers"
    );
}

/// The blocking and non-blocking producers are the same claim, in both orders.
#[test]
fn conveyor_feeder_claim_covers_the_blocking_variant() {
    static PLAIN_FIRST: Conveyor<f32, 8> = Conveyor::new();
    static BLOCKING_FIRST: Conveyor<f32, 8> = Conveyor::new();

    assert!(PLAIN_FIRST.feeder().is_some());
    assert!(
        PLAIN_FIRST.blocking_feeder().is_none(),
        "a blocking feeder after a plain one would give the queue two producers"
    );

    assert!(BLOCKING_FIRST.blocking_feeder().is_some());
    assert!(
        BLOCKING_FIRST.feeder().is_none(),
        "a plain feeder after a blocking one would give the queue two producers"
    );
}

/// The blocking and non-blocking consumers are the same claim, in both orders.
#[test]
fn conveyor_picker_claim_covers_the_blocking_variant() {
    static PLAIN_FIRST: Conveyor<f32, 8> = Conveyor::new();
    static BLOCKING_FIRST: Conveyor<f32, 8> = Conveyor::new();

    assert!(PLAIN_FIRST.picker().is_some());
    assert!(
        PLAIN_FIRST.blocking_picker().is_none(),
        "a blocking picker after a plain one would give the queue two consumers"
    );

    assert!(BLOCKING_FIRST.blocking_picker().is_some());
    assert!(
        BLOCKING_FIRST.picker().is_none(),
        "a plain picker after a blocking one would give the queue two consumers"
    );
}

/// The poster must tolerate publishing many times with no read in between. That is
/// the normal regime: the writing context runs at the sampling rate while the reader
/// runs once per control period.
///
/// Every publication swaps the poster's private slot index into the shared word,
/// whose top bit also flags "there is a new edition". If either side lets that flag
/// survive into its private index, the index stops naming a slot.
#[test]
fn billboard_accepts_repeated_publication_without_a_read() {
    let billboard: Billboard<f32> = Billboard::new();
    let mut poster = billboard
        .poster()
        .expect("the first poster must be available");
    let mut sim = EndlessSimulation::new(1.0);

    for edition in 1..=5 {
        poster.block(edition as f32, sim.next().unwrap());
    }
}

/// Between publications the viewer must keep handing back the latest edition.
///
/// The viewer may only take a new slot when the shared word says one is waiting. A
/// viewer that exchanged slots unconditionally would rotate through all three and
/// eventually return a value older than one it had already returned — a state the
/// plant was never in.
#[test]
fn billboard_viewer_never_goes_backwards() {
    let billboard: Billboard<f32> = Billboard::new();
    let mut poster = billboard
        .poster()
        .expect("the first poster must be available");
    let mut viewer = billboard
        .viewer()
        .expect("the first viewer must be available");
    let mut sim = EndlessSimulation::new(1.0);

    poster.block(7.0, sim.next().unwrap());

    let observed: Vec<Option<f32>> = (0..4)
        .map(|_| viewer.block((), sim.next().unwrap()))
        .collect();

    assert!(
        observed.iter().all(|edition| *edition == Some(7.0)),
        "one publication followed by four reads returned {observed:?}"
    );
}
