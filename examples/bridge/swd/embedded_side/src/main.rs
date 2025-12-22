#![no_std]
#![no_main]

use aule::prelude::*;
use cortex_m_rt::entry;
use defmt::println;
use defmt_rtt as _;
use embedded_alloc::LlffHeap as Heap;
use microbit as _;
use panic_probe as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    init_heap();

    let mut swd_conn = SwdConnection::default();

    let time = EndlessTime::continuous(1e-3);
    let mut pid = PID::new(40.0, 10.0, 10.0);
    let mut error = swd_conn.new_bridge_down("pid1").unwrap();
    let mut plant = swd_conn.new_bridge_up("pid1").unwrap();

    let error_addr = &error as *const _ as usize;
    println!("error address: 0x{:x}", error_addr);

    let plant_addr = &plant as *const _ as usize;
    println!("plant address: 0x{:x}", plant_addr);

    for dt in time {
        let error_signal = error.output(dt);
        let control_signal = error_signal * pid.as_block();
        plant.output(control_signal);
    }

    unreachable!();
}

const HEAP_SIZE: usize = 1024;

fn init_heap() {
    use core::mem::MaybeUninit;

    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) };
}
