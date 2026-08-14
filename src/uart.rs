// Драйвер UART0 для платформи LM3S6965 (QEMU)
use core::ptr;

const UART0_BASE: usize = 0x4000_C000;

const UART_DR: usize = UART0_BASE + 0x000;   // Data register
const UART_FR: usize = UART0_BASE + 0x018;   // Flag register
const UART_IBRD: usize = UART0_BASE + 0x024; // Integer baud-rate divisor
const UART_FBRD: usize = UART0_BASE + 0x028; // Fractional baud-rate divisor
const UART_LCRH: usize = UART0_BASE + 0x02C; // Line control
const UART_CTL: usize = UART0_BASE + 0x030;  // Control
const UART_IM: usize = UART0_BASE + 0x038;   // Interrupt mask

// Bits у регістрі UART_FR
const UART_FR_RXFE: u32 = 0x10; // Receive FIFO empty
const UART_FR_TXFF: u32 = 0x20; // Transmit FIFO full

/// Ініціалізація UART0: 115200 бод, 8N1, FIFO увімкнено.
pub fn init() {
    unsafe {
        // Вимикаємо UART
        ptr::write_volatile(UART_CTL as *mut u32, 0);

        // Налаштовуємо швидкість: 20 МГц / (16 * 115200) ≈ 10.85
        // IBRD = 10, FBRD = 54 (дрібна частина 0.85 * 64)
        ptr::write_volatile(UART_IBRD as *mut u32, 10);
        ptr::write_volatile(UART_FBRD as *mut u32, 54);

        // 8 біт даних, без паритету, 1 стоп-біт, FIFO увімкнено
        ptr::write_volatile(UART_LCRH as *mut u32, 0x70);

        // Увімкнути UART, передавач і приймач
        ptr::write_volatile(UART_CTL as *mut u32, 0x301);

        // Вимкнути всі переривання UART
        ptr::write_volatile(UART_IM as *mut u32, 0);
    }
}

/// Відправити один символ.
pub fn putc(c: u8) {
    unsafe {
        // Чекаємо, поки FIFO передавача не буде повним
        while ptr::read_volatile(UART_FR as *const u32) & UART_FR_TXFF != 0 {}
        ptr::write_volatile(UART_DR as *mut u32, c as u32);
    }
}

/// Відправити рядок.
pub fn puts(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}

/// Отримати один символ (блокується, поки немає даних).
pub fn getc() -> u8 {
    unsafe {
        // Чекаємо, поки FIFO приймача не буде порожнім
        while ptr::read_volatile(UART_FR as *const u32) & UART_FR_RXFE != 0 {}
        ptr::read_volatile(UART_DR as *const u32) as u8
    }
}
