// This module contains various constants that are
// platform specific for arm1176

use register::Register;
use vital;
use vital::Vital;
use core::mem;
use ascii;
use screen;

#[derive(Clone,
         Copy)]
#[allow(dead_code)]
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

#[allow(dead_code)]
#[repr(u32)]
enum CLCDC {
    CursorImage = 0x10120800,
    CursorControlRegister = 0x10120C00,
    CursorConfigurationRegister = 0x10120C04,
    CursorPalette01 = 0x10120C08,
    CursorPalette02 = 0x10120C0C,
    CursorPositionRegister = 0x10120C10,
    CursorClipPositionRegister = 0x10120C14,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
enum TimerControlRegisterBits {
    OneShot = 0,
    TimerSize = 1,
    TimerPre0 = 2,
    TimerPre1 = 3,
    IntEnable = 5,
    TimerMode = 6,
    TimerEn = 7
}

#[allow(dead_code)]
enum TimerMode {
    FreeRunning = 0,
    Periodic = 1,
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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
fn set_irq_handler(src: InterruptSources, handler: fn(vital: &mut Vital, lr_irq: u32) -> u32) -> () {
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
    Register::new(TimerModules::Timer0Load as u32 as *mut u32).set(0x100);

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

#[allow(dead_code)]
pub enum InterruptType {
    IRQ_ENABLED,
    DISABLED
}

#[inline]
pub fn switch_to_sys_mode(interrupts: InterruptType) {
    // let's construct a word.
    let interrupt_mode: i32 = match interrupts {
        InterruptType::DISABLED => 0xdf, // user mode, interrupts dissabled
        _ => 0x1f
    };

    unsafe {
        asm!("msr cpsr_c, $0" :: "i"(interrupt_mode));
    }
}

#[inline]
pub fn switch_to_irq_mode(interrupts: InterruptType) {
    // let's construct a word.
    let interrupt_mode: i32 = match interrupts {
        InterruptType::DISABLED => 0xd2, // user mode, interrupts dissabled
        _ => 0x12
    };

    unsafe {
        asm!("msr cpsr_c, $0" :: "i"(interrupt_mode));
    }
}

#[no_mangle]
extern {
    fn asm_save_context(registers: *const u32) -> ();
    fn asm_restore_context(registers: *const u32) -> ();
}

#[inline]
pub fn save_context_to_stack(registers: &mut [u32; 17]) {
    unsafe {
        //  llvm inline asm. is just not good enough. We will just place
        //  call the asm method and be done with it
        //  move irq mode sp
        asm_save_context(&registers[0] as *const u32);
    }
}

#[inline]
pub fn restore_context_from_stack(registers: &mut [u32; 17]) {
    unsafe {
        //  llvm inline asm. is just not good enough. We will just place
        //  call the asm method and be done with it
        //  move irq mode sp
        asm_restore_context(&registers[0] as *const u32);
    }
}

#[inline]
pub fn save_sp_to_process(stack_pointer: &mut u32) {
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

extern {
    fn asm_set_vital_instance (vital: &Vital) -> ();
}

#[inline]
pub fn set_vital_instance (vital: &Vital) {
    unsafe {
        asm_set_vital_instance(vital);
    }
}

extern {
    fn asm_induce_swi (number: u32, value_1: u32, value_2: u32) -> u32;
}

#[no_mangle]
pub fn swi(number: u32, value_1: u32, value_2: u32) -> u32 {
    unsafe {
        asm_induce_swi(number, value_1, value_2)
    }
}

extern {
    // because of not supporting armvk6, this is implemented
    // in asm.
    fn wait_for_event () -> ();
}

#[inline]
pub fn wfe () -> () {
    unsafe {
        wait_for_event();
    }
}


pub fn write_cursor () -> () {
/*
    let control = CLCDC::CursorControlRegister as u32 as *mut u32;
    Register::new(control).set(0x1); // turn on the cursor

    let palette = CLCDC::CursorPalette01 as u32 as *mut u32;
    Register::new(palette).set(0x00FFFFFF);

    let position = CLCDC::CursorPositionRegister as u32 as *mut u32;
    Register::new(position).set(0x000F000F);

    let register_address = CLCDC::CursorImage as u32 as *mut u32;

    loop {
        let image = 0x0000;
        Register::new(register_address).set(image);
    }
*/
    Register::new(0x1000001C as *mut u32).set(0x2c77);
    Register::new(0x10120000 as *mut u32).set(0x3f1f3f9c); //0x1313a4c4);
    Register::new(0x10120004 as *mut u32).set(0x090B61df); //0x0505f657);
    Register::new(0x10120008 as *mut u32).set(0x067f1800); //0x071f1800);
    Register::new(0x10120010 as *mut u32).set(1024 * 1024);
    Register::new(0x10120018 as *mut u32).set(0x82b);

    /* loop {
        Register::new((1024 * 1024 + 1) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 2) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 3) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 4) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 5) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 6) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 7) as *mut u32).set(0xFFFFFF);
        
        Register::new((1024 * 1024 + 100 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 101 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 102 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 103 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 104 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 106 + 1600 *100) as *mut u32).set(0xFFFFFF);
        Register::new((1024 * 1024 + 107 + 1600 *100) as *mut u32).set(0xFFFFFF);
    } */

    ascii::putchar(b'm', 128, 64, 0xffffff);
    ascii::putchar(b'i', 128 + (8 * 4) * 1, 64, 0xffffff);
    ascii::putchar(b'c', 128 + (8 * 4) * 2, 64, 0xFFFFFF);
    ascii::putchar(b'o', 128 + (8 * 4) * 3, 64, 0xFFFFFF);

    screen::print_something();
}
