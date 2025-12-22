#![no_std]
#![no_main]

use aule::prelude::*;
use cortex_m_rt::entry;
use defmt_rtt as _;
use microbit as _;
use panic_probe as _;

use defmt::println;
use embedded_alloc::TlsfHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

const HEAP_SIZE: usize = 1024;

#[entry]
fn main() -> ! {
    init_heap();

    let time = Time::continuous(1e-3, 10.0);

    let mut step = Step::default();
    let mut pid = PID::new(40.0, 10.0, 10.00);
    let mut plant = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).to_ss_controllable(RK4);
    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut printer = DefmtPrinter;

    println!("Starting simulation...");

    for dt in time {
        let input = dt * step.as_block();
        let error = input - plant.last_output();
        iae.output(error);
        ise.output(error);
        itae.output(error);

        let control_signal = error * pid.as_block();
        let output = plant.output(control_signal);

        let printer_input = input.map(|i| [i, output.value]);
        printer.output(printer_input);
    }

    println!("IAE Value: {}", iae.value());
    println!("ISE Value: {}", ise.value());
    println!("ITAE Value: {}", itae.value());

    loop {
        cortex_m::asm::nop();
    }
}

struct DefmtPrinter;

impl Block for DefmtPrinter {
    type Input = [f64; 2];
    type Output = [f64; 2];
    type TimeType = Continuous;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        println!(
            "Time: {}, Input: {}, Output: {}",
            input.delta.sim_time().as_secs_f64(),
            input.value[0],
            input.value[1]
        );
        input
    }
}

fn init_heap() {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
}
