
use ruxpin_kernel::irqs;
use ruxpin_kernel::notice;
use ruxpin_kernel::proc::scheduler;
use ruxpin_kernel::arch::KernelVirtualAddress;
use ruxpin_kernel::misc::deviceio::DeviceRegisters;

mod registers {
    pub const CONTROL: usize = 0x00;
    pub const COUNT_LOW: usize = 0x04;
    pub const COMPARE_1: usize = 0x10;
}

/// システムタイマーレジスタ
const SYS_TIMER: DeviceRegisters<u32> = DeviceRegisters::new(KernelVirtualAddress::new(0x3F00_3000));

/// システムタイマー構造体
pub struct SystemTimer;

impl SystemTimer {
    /// 初期化. プリエンプションに使用
    pub fn init(irq: usize) {
        notice!("timer: initializing generic arm timer to trigger context switch");

        irqs::register_irq(irq ,SystemTimer::handle_irq).unwrap();
        irqs::enable_irq(irq);

        // 20_000 は 20ms で system clock は 1MHz ということか?
        unsafe {
            let value = SYS_TIMER.get(registers::COUNT_LOW) + 20000;
            SYS_TIMER.set(registers::COMPARE_1, value);
        }
    }

    /// システムタイマーをリセット
    pub fn reset() {
        unsafe {
            SYS_TIMER.set(registers::CONTROL, 1 << 1);
            let value = SYS_TIMER.get(registers::COUNT_LOW) + 20000;
            SYS_TIMER.set(registers::COMPARE_1, value);
        }
    }

    /// システムタイマーの割り込みハンドラ
    fn handle_irq() {
        SystemTimer::reset();
        scheduler::schedule();
    }
}

