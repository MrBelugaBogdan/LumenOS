#![no_std]
#![no_main]

mod uart;

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    uart::init();
    uart::puts("LumenOS v0.1.0\n");
    uart::puts("Type 'help' for commands.\n> ");

    let mut line = [0u8; 64];
    let mut idx = 0;

    loop {
        let c = uart::getc();

        // Ехо-вивід символу
        uart::putc(c);

        if c == b'\r' || c == b'\n' {
            uart::putc(b'\n'); // перехід на новий рядок
            process_command(&line[..idx]);
            idx = 0;
            uart::puts("> ");
        } else {
            if idx < line.len() - 1 {
                line[idx] = c;
                idx += 1;
            }
        }
    }
}

/// Обробляє введену команду.
fn process_command(line: &[u8]) {
    let cmd = core::str::from_utf8(line).unwrap_or("");
    match cmd.trim() {
        "help" => uart::puts("Commands: help, about, echo <text>\n"),
        "about" => uart::puts("LumenOS - a minimal wearable OS kernel\n"),
        _ if cmd.starts_with("echo ") => {
            uart::puts(&cmd[5..]);
            uart::puts("\n");
        }
        _ => uart::puts("Unknown command\n"),
    }
}
