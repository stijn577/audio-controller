use defmt::warn;
use embassy_stm32::{
    rcc::{
        AHBPrescaler, APBPrescaler, Hse, HseMode, LsConfig, Pll, PllMul, PllPDiv, PllPreDiv,
        PllQDiv, PllRDiv, PllSource, RtcClockSource, Sysclk,
    },
    time::Hertz,
};

#[global_allocator]
static HEAP_ALLOCATOR: alloc_cortex_m::CortexMHeap = alloc_cortex_m::CortexMHeap::empty();

#[allow(unsafe_code)]
pub fn heap_setup() {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 50_000;
    unsafe { HEAP_ALLOCATOR.init(start, size) }
}

pub fn clock_setup(config: &mut embassy_stm32::rcc::Config) {
    config.hsi = true;
    config.hse = Some(Hse {
        freq: Hertz(25_000_000),
        mode: HseMode::Oscillator,
    });
    config.sys = Sysclk::PLL1_P;
    config.pll_src = PllSource::HSE;
    config.pll = Some(Pll {
        prediv: PllPreDiv::DIV25,
        mul: PllMul::MUL336,
        divp: Some(PllPDiv::DIV4),
        divq: Some(PllQDiv::DIV7),
        divr: None,
    });
    config.plli2s = Some(Pll {
        prediv: PllPreDiv::DIV25,
        mul: PllMul::MUL192,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
    config.ahb_pre = AHBPrescaler::DIV1;
    config.apb1_pre = APBPrescaler::DIV2;
    config.apb2_pre = APBPrescaler::DIV1;

    warn!("config.ls = ls; gives issues for some reason, not yet implemented, so we use default for RTC for now.")

    // config.ls = ls;
}
