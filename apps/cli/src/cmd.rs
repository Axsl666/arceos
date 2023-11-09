use core::time;
use std::{
    io::{self},
    thread::sleep,
};

#[cfg(all(not(feature = "axstd"), unix))]

macro_rules! print_err {
    ($cmd: literal, $msg: expr) => {
        println!("{}: {}", $cmd, $msg);
    };
    ($cmd: literal, $arg: expr, $err: expr) => {
        println!("{}: {}: {}", $cmd, $arg, $err);
    };
}

type CmdHandler = fn(&str);

const CMD_TABLE: &[(&str, CmdHandler)] = &[
    ("exit", do_exit),
    ("help", do_help),
    ("uname", do_uname),
    ("ldr", do_ldr),
    ("str", do_str),
    ("uart", do_uart),
    ("test", do_test),
];

fn do_uname(_args: &str) {
    let arch = option_env!("AX_ARCH").unwrap_or("");
    let platform = option_env!("AX_PLATFORM").unwrap_or("");
    let smp = match option_env!("AX_SMP") {
        None | Some("1") => "",
        _ => " SMP",
    };
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0");
    println!(
        "ArceOS {ver}{smp} {arch} {plat}",
        ver = version,
        smp = smp,
        arch = arch,
        plat = platform,
    );
}

fn do_help(_args: &str) {
    println!("Available commands:");
    for (name, _) in CMD_TABLE {
        println!("  {}", name);
    }
}

fn do_exit(_args: &str) {
    println!("Bye~");
    std::process::exit(0);
}

fn do_ldr(args: &str) {
    println!("ldr");
    if args.is_empty() {
        println!("try: ldr ffff0000400fe000 / ldr ffff000040080000 ffff000040080008");
    }

    fn ldr_one(addr: &str) -> io::Result<()> {
        println!("addr = {}", addr);

        if let Ok(parsed_addr) = u64::from_str_radix(addr, 16) {
            let address: *const u32 = parsed_addr as *const u32; // 强制转换为合适的指针类型

            let value: u32;
            println!("Parsed address: {:p}", address); // 打印地址时使用 %p 格式化符号

            unsafe {
                value = *address;
            }

            println!("Value at address {}: 0x{:X}", addr, value); // 使用输入的地址打印值
        } else {
            println!("Failed to parse address.");
        }
        return Ok(());
    }

    fn ldr_n(addr: &str, n: &str) -> io::Result<()> {
        println!("addr = {}", addr);

        if let Ok(parsed_addr) = u64::from_str_radix(addr, 16) {
            if let Ok(parsed_n) = u64::from_str_radix(n, 10) {
                let address: *const u32 = parsed_addr as *const u32; // 强制转换为合适的指针类型
                println!("Parsed address: {:p}", address); // 打印地址时使用 %p 格式化符号

                for i in 0..parsed_n {
                    let round_address = (parsed_addr + 4 * i) as *const u32;
                    let value: u32;
                    unsafe {
                        value = *round_address;
                    }

                    println!("Value at address {:p}: 0x{:X}", round_address, value);
                    // 使用输入的地址打印值
                }
            } else {
                println!("Failed to parse num.");
            }
        } else {
            println!("Failed to parse address.");
        }
        return Ok(());
    }

    let mut arg_iter = args.split_whitespace();

    if let Some(addr) = arg_iter.next() {
        if let Some(num) = arg_iter.next() {
            if arg_iter.next().is_none() {
                if let Err(e) = ldr_n(addr, num) {
                    println!("ldr {} {} {}", addr, num, e);
                }
            } else {
                println!("too many arugments!");
            }
        } else {
            if let Err(e) = ldr_one(addr) {
                println!("ldr {} {}", addr, e);
            }
        }
    } else {
        println!("Missing address arugment !");
    }
}

// use crate::mem::phys_to_virt;
// use core::ptr::{read_volatile, write_volatile};

fn do_str(args: &str) {
    println!("str");
    if args.is_empty() {
        println!("try: str ffff0000400fe000 12345678");
    }

    fn str_one(addr: &str, val: &str) -> io::Result<()> {
        println!("addr = {}", addr);
        println!("val = {}", val);

        if let Ok(parsed_addr) = u64::from_str_radix(addr, 16) {
            let address: *mut u32 = parsed_addr as *mut u32; // 强制转换为合适的指针类型
            println!("Parsed address: {:p}", address); // 打印地址时使用 %p 格式化符号

            if let Ok(parsed_val) = u32::from_str_radix(val, 16) {
                let value: u32 = parsed_val; // 不需要将值转换为指针类型
                println!("Parsed value: 0x{:X}", value); // 直接打印解析的值

                // let ptr = phys_to_virt(parsed_addr.into()).as_mut_ptr() as *mut u32;
                unsafe {
                    *address = value;
                    // write_volatile(address, value);
                    // write_volatile(ptr, value);
                }

                println!("Write value at address {}: 0x{:X}", addr, value); // 使用输入的地址打印值
            }
        } else {
            println!("Failed to parse address.");
        }

        Ok(())
    }

    let mut split_iter = args.split_whitespace();

    if let Some(addr) = split_iter.next() {
        println!("First element: {}", addr);

        if let Some(val) = split_iter.next() {
            println!("Second element: {}", val);
            str_one(addr, val).unwrap(); // 调用 str_one 函数并传递 addr 和 val
        }
    }
}

fn do_uart(args: &str) {
    match args {
        "5" => {
            // let str_addr0 = "ffff0000fe200000 1B";
            let str_addr1 = "ffff0000fe200004 246c0";
            let str_addr2 = "ffff0000fe2000e4 55000000";
            let str_addr3 = "ffff0000fe201a24 1A";
            let str_addr4 = "ffff0000fe201a28 3";
            let str_addr5 = "ffff0000fe201a2c 70";
            let str_addr6 = "ffff0000fe201a30 301";
            //调用str写入函数
            //do_str(str_addr0);
            do_str(str_addr1);
            do_str(str_addr2);
            do_str(str_addr3);
            do_str(str_addr4);
            do_str(str_addr5);
            do_str(str_addr6);
        }
        _ => {}
    }
}

fn do_test(args: &str) {
    fn delay(seconds: u64) {
        for i in 1..seconds + 1 {
            fn fibonacci_recursive(n: u64) -> u64 {
                if n == 0 {
                    return 0;
                }
                if n == 1 {
                    return 1;
                }
                return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
            }
            fibonacci_recursive(36 + (i % 2));
        }
    }
    if args == "run" {
        loop {
            let arges = "ffff0000fe201a00 41";
            do_str(arges);
            delay(4);
        }
    }
}

pub fn run_cmd(line: &[u8]) {
    let line_str = unsafe { core::str::from_utf8_unchecked(line) };
    let (cmd, args) = split_whitespace(line_str);
    if !cmd.is_empty() {
        for (name, func) in CMD_TABLE {
            if cmd == *name {
                func(args);
                return;
            }
        }
        println!("{}: command not found", cmd);
    }
}

fn split_whitespace(str: &str) -> (&str, &str) {
    let str = str.trim();
    str.find(char::is_whitespace)
        .map_or((str, ""), |n| (&str[..n], str[n + 1..].trim()))
}

fn init_pcie() {
    // set bits 0 and 1 of 32 bit register [0xfd509210] (reset controller)
}
