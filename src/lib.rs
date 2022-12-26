use std::{path::PathBuf, fmt::format, fs, io::Read};

use ansi_term::Colour;

#[derive(Debug)]
pub struct VMOptions {
    pub rom_binary: String,
    pub debug: bool,
}

/**
 * Represents the SIS16 Virtual Machine
 */
#[derive(Debug)]
pub struct VirtualMachine {
    options: VMOptions,
    rom: [u8; 0xFFFF],
    ram: [u8; 0xFFFF],
    program_counter: Register,
    stack_pointer: u8,
    flags: CPUFlags,
    memory_address_register: Register,
    instruction_register: Register,
    micro_instruction_counter: u8,
}

#[derive(Debug)]
struct CPUFlags {
    carry: bool,
    zero: bool,
}

impl CPUFlags {
    fn new() -> CPUFlags {
        CPUFlags {
            carry: false,
            zero: false,
        }
    }
}

#[derive(Debug)]
struct Register {
    pub value: u16,
    value_out: bool,
    value_in: bool,
}

impl Register {
    fn new() -> Register {
        Register {
            value: 0,
            value_out: false,
            value_in: false,
        }
    }
}

impl VirtualMachine {
    pub fn new(options: VMOptions) -> VirtualMachine {
        let path = PathBuf::from(&options.rom_binary);

         // Check if input file exists
        if !&path.exists() {
            report_error(format!("Path {path:?} does not exist!"));
        }

        // Read entire file
        let content = fs::read(&path).expect("Could not read file");
        
        if content.len() != 0xFFFF {
            report_error(format!("File {:?} must be exactly 65535 (0xFFFF) bytes!", options.rom_binary));
        }

        let mut rom = [0u8; 0xFFFF];

        content.as_slice().read(&mut rom).expect("Failed to write file content into rom array");

        VirtualMachine {
            options,
            rom,
            ram: [0; 0xFFFF],
            program_counter: Register::new(),
            stack_pointer: 0,
            flags: CPUFlags::new(),
            memory_address_register: Register::new(),
            instruction_register: Register::new(),
            micro_instruction_counter: 0
        }
    }

    fn cycle_clock(&self) {}

    fn compute_register_values(&mut self) {
        let bus = self.get_bus_value();

        if self.program_counter.value_in {
            self.program_counter.value = bus;
        }

        if self.memory_address_register.value_in {
            self.memory_address_register.value = bus;
        }

        if self.instruction_register.value_in {
            self.instruction_register.value = bus;
        }
    }

    fn get_bus_value(&self) -> u16 {
        let mut value = 0u16;

        if self.program_counter.value_out {
            value |= self.program_counter.value;
        }
        if self.memory_address_register.value_out {
            value |= self.memory_address_register.value;
        }
        if self.instruction_register.value_out {
            value |= self.instruction_register.value;
        }

        value
    }
    
}

pub fn report_error<S: Into<String>>(
    error: S
) -> ! {
    // Print error message
    eprintln!(
        "{} {}",
        Colour::Red.bold().paint("[ERROR]"),
        Colour::Red.paint(error.into())
    );

    // Exit with non-zero code to signal an error occurred
    std::process::exit(1);
}