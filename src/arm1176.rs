// This module contains various constants that are
// platform specific for arm1176

use register::Register;
use vital;

#[derive(Clone,
         Copy)]
enum InterruptSources {
    WatchDog = 0,
    Software = 1,
    CommsRx = 2,
    CommsTx,
    Timers01,
    Timers23,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    RTC,
    SSP,
    UART0,
    UART1,
    UART2,
    SCI0,
    CLCDC,
    DMA,
    PWRFAIL,
    MBX,
    GND,
    VICINTSOURCE21,
    VICINTSOURCE22,
    VICINTSOURCE23,
    VICINTSOURCE24,
    VICINTSOURCE25,
    VICINTSOURCE26,
    VICINTSOURCE27,
    VICINTSOURCE28,
    VICINTSOURCE29,
    VICINTSOURCE30,
    VICINTSOURCE31,
}

enum TimerModules {
    Timer0Load = 0x101E2000,
    Timer0Value = 0x101E2004,
    Timer0Control = 0x101E2008,
    Timer0IntClr = 0x101E200C,
    Timer0RIS = 0x101E2010,
    Timer0MIS = 0x101E2014,
    Timer0BGLoad = 0x101E2018,
    Timer1Load = 0x101E2020,
    Timer1Value = 0x101E2024,
    Timer1Control = 0x101E2028,
    Timer1IntClr = 0x101E202C,
    Timer1RIS = 0x101E2030,
    Timer1MIS = 0x101E2034,
    Timer1BGLoad = 0x101E2038,
}

enum TimerControlRegisterBits {
    OneShot = 0,
    TimerSize = 1,
    TimerPre0 = 2,
    TimerPre1 = 3,
    IntEnable = 5,
    TimerMode = 6,
    TimerEn = 7
}

enum TimerMode {
    FreeRunning = 0,
    Periodic = 1,
}

enum PrimaryInterruptControllerMap {
    PICIRQStatus = 0x10140000,
    PICFIQStatus = 0x10140004,
    PICRawIntr = 0x10140008,
    PICIntSelect = 0x1014000C,
    PICIntEnable = 0x10140010,
    PICIntEnClear = 0x10140014,
    PICSoftInt = 0x10140018,
    PICSoftIntClear = 0x1014001C,
    PICProtection = 0x10140020,
    PICVectAddr = 0x10140030,
    PICDefVectAddr = 0x10140034,
    PICVectAddr0 = 0x10140100,
    PICVectAddr1 = 0x10140104,
    PICVectAddr2 = 0x10140108,
    PICVectAddr3 = 0x1014010C,
    PICVectAddr4 = 0x10140110,
    PICVectAddr5 = 0x10140114,
    PICVectAddr6 = 0x10140118,
    PICVectAddr7 = 0x1014011C,
    PICVectAddr8 = 0x10140120,
    PICVectAddr9 = 0x10140124,
    PICVectAddr10 = 0x10140128,
    PICVectAddr11 = 0x1014012C,
    PICVectAddr12 = 0x10140130,
    PICVectAddr13 = 0x10140134,
    PICVectAddr14 = 0x10140138,
    PICVectAddr15 = 0x1014013C,
    PICVectCntl0 = 0x10140200,
    PICVectCntl1 = 0x10140204,
    PICVectCntl2 = 0x10140208,
    PICVectCntl3 = 0x1014020C,
    PICVectCntl4 = 0x10140210,
    PICVectCntl5 = 0x10140214,
    PICVectCntl6 = 0x10140218,
    PICVectCntl7 = 0x1014021C,
    PICVectCntl8 = 0x10140220,
    PICVectCntl9 = 0x10140224,
    PICVectCntl10 = 0x10140228,
    PICVectCntl11 = 0x1014022C,
    PICVectCntl12 = 0x10140230,
    PICVectCntl13 = 0x10140234,
    PICVectCntl14 = 0x10140238,
    PICVectCntl15 = 0x1014023C,
    PICITCR = 0x10140300,
    PICITIP1 = 0x10140304,
    PICITIP2 = 0x10140308,
    PICITOP1 = 0x1014030C,
    PICITOP2 = 0x10140310,
    PICPeriphID0 = 0x10140FE0,
    PICPeriphID1 = 0x10140FE4,
    PICPeriphID2 = 0x10140FE8,
    PICPeriphID3 = 0x10140FEC,
    PICPCellID0 = 0x10140FF0,
    PICPCellID1 = 0x10140FF4,
    PICPCellID2 = 0x10140FF8,
    PICPCellID3 = 0x10140FFC,
}

// real time clock registers
enum RTCRegisters {
    RTCDR =  0x101E8000 // data register
}

// enable interrupts
#[inline]
fn enable_interrupts(interrupt: InterruptSources) -> () {
    let interrupt_bit = 1 << interrupt as u32;

    // connect vector with the source and enable it
    let int_contr_reg = Register::new((PrimaryInterruptControllerMap::PICVectCntl0 as u32 + 4 * (interrupt as u32)) as *mut u32);
    int_contr_reg.set(1 << 5 | interrupt as u32);

    // Write 1 into Interrupt Enable Register
    let int_enable_reg = Register::new(PrimaryInterruptControllerMap::PICIntEnable as u32 as *mut u32);

    int_enable_reg.set(interrupt_bit);

    // enable notification
    set_irq_control(interrupt, 1);
}

// sets the address of the interrupt handler
#[inline]
fn set_irq_handler(src: InterruptSources, handler: fn() -> ()) -> () {
    let reg = PrimaryInterruptControllerMap::PICVectAddr0 as u32 + 4*src as u32;
    let register_address = reg as *mut u32;

    Register::new(register_address).set(handler as u32);
}

// sets the value in the irq control register
fn set_irq_control(src: InterruptSources, value: u32) -> () {
    let reg = PrimaryInterruptControllerMap::PICVectCntl0 as u32 + src as u32;
    let register_address = reg as *mut u32;

    Register::new(register_address).set(value);
}

// setup the timer
#[inline]
fn setup_timer0() -> () {
    Register::new(TimerModules::Timer0Load as u32 as *mut u32).set(0x10);

    let timer_cntrl_reg = Register::new(TimerModules::Timer0Control as u32 as *mut u32);
    let old_timer_control_value = timer_cntrl_reg.get();

    let timer_control_value = old_timer_control_value |
                              (1 << TimerControlRegisterBits::TimerMode as u32) |
                              (1 << TimerControlRegisterBits::TimerEn as u32);

    timer_cntrl_reg.set(timer_control_value);
}


#[no_mangle]
pub fn enable_timer_interrupt() -> () {
    set_irq_handler(InterruptSources::Timers01, vital::timer_interrupt_routine);
    setup_timer0();
    enable_interrupts(InterruptSources::Timers01);

    enable_irq_interrupts();
}

#[inline]
pub fn get_current_time() -> u32 {
    Register::new(RTCRegisters::RTCDR as u32 as *mut u32).get()
}

pub enum InterruptType {
    IRQ_ENABLED,
    DISABLED
}

#[inline]
pub fn switch_to_user_mode(interrupts: InterruptType) {
    // let's construct a word.
    let interrupt_mode: i32 = match (interrupts) {
        InterruptType::DISABLED => 0xd0, // user mode, interrupts dissabled
        _ => 0x10
    };

    unsafe {
        asm!("msr cpsr_c, $0" :: "i"(interrupt_mode));
    }
}

#[inline]
pub fn save_context_to_stack() {
    unsafe {
        asm("stmfd r13!, {r0-r15}");
    }
}

#[inline]
pub fn save_sp_to_process(stack_pointer: &mut i32) {
    unsafe {
        asm!("str r13, $0" :: "=*m"(stack_pointer));
    }
}

#[inline]
fn enable_irq_interrupts() -> ()
{
    unsafe {
        asm!("mrs r2, cpsr");
        asm!("bic r2, #0x80");
        asm!("msr cpsr_cxsf, r2");
    }
}
