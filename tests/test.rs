extern crate serde;
extern crate serde_yaml;

mod test_utils;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_lda_tests {
    use std::fs::File;
    use std::io::prelude::*;
    use serde::{Serialize, Deserialize};
    use std::collections::BTreeMap;
    use serde_yaml;
    use crate::test_utils::*;
    use nesrs::cpu::*;
    use nesrs::cpu::types::*;
    use nesrs::memory::*;


    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct InitMemory {
        start: usize,
        values: Vec<u8>
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CPUMock {
        a: Option<u8>,
        x: Option<u8>,
        y: Option<u8>,
        sp: Option<u8>,
        p: Option<u8>,
        pc: Option<u16>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Case {
        mode: String,
        code: Vec<u8>,
        expected_value: u16,
        target_address: usize,
        read_from: String,
        cycles: Option<u32>,
        cpu: Option<CPUMock>,
        check_n: Option<bool>,
        check_z: Option<bool>,
        check_c: Option<bool>,
        init_memory_value: Option<u8>,
        init_memories: Option<InitMemory>
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct OpcodeCase {
        cases: Vec<Case>
    }

    #[test]
    fn test() -> std::io::Result<()> {
        let mut file = File::open("./tests/testlist.yml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let deserialized_map: BTreeMap<String, OpcodeCase> = serde_yaml::from_str(&contents).unwrap();
        for (key, value) in deserialized_map.iter() {
            println!("[TEST] Opcode {}", key);

            for case in &value.cases {
                println!("[TEST]   > in addressing mode: {}", case.mode);

                let mut cpu = CPU::new();

                let mut memory = RAM::new();
                set_reset!(memory, 0x8001);

                cpu.reset();
                loop_cpu!(cpu, memory);


                if let Some(regs) = &case.cpu {
                    cpu.regs.a = regs.a.unwrap_or(0);
                    cpu.regs.x = regs.x.unwrap_or(0);
                    cpu.regs.y = regs.y.unwrap_or(0);
                }

                if let Some(value) = &case.init_memory_value {
                    memory.write(case.target_address, *value);
                }

                if let Some(config) = &case.init_memories {
                    let mut offset = 0;
                    for v in &config.values {
                        memory.write(config.start + offset, *v);
                        offset += 1;
                    }
                }

                let prev_cycle = cpu.total_cycles;
                set_ram_from_vec!(memory, 0x8001, &case.code);

                loop_cpu!(cpu, memory);

                match &case.read_from[..] {
                    "a" => {
                        assert_eq!(cpu.regs.a, case.expected_value as u8);
                    }
                    "x" => {
                        assert_eq!(cpu.regs.x, case.expected_value as u8);
                    }
                    "y" => {
                        assert_eq!(cpu.regs.y, case.expected_value as u8);
                    }
                    "sp" => {
                        assert_eq!(cpu.regs.sp, case.expected_value as u8);
                    }
                    "p" => {
                        assert_eq!(cpu.regs.p.bits(), case.expected_value as u8);
                    }
                    "pc" => {
                        assert_eq!(cpu.regs.pc, case.expected_value);
                    }
                    "address" => {
                        assert_eq!(memory.read(case.target_address, false), case.expected_value as u8);
                    }
                    _ => {}
                }

                if let Some(expected_cycle) = case.cycles {
                    assert_eq!(cpu.total_cycles - prev_cycle, expected_cycle);
                }

                if let Some(expected_value) = case.check_n {
                    println!("[TEST]      > Test N flag");
                    assert_eq!(cpu.regs.p.contains(StatusFlag::N), expected_value);
                }

                if let Some(expected_value) = case.check_z {
                    println!("[TEST]      > Test Z flag");
                    assert_eq!(cpu.regs.p.contains(StatusFlag::Z), expected_value);
                }

                if let Some(expected_value) = case.check_c {
                    println!("[TEST]      > Test C flag");
                    assert_eq!(cpu.regs.p.contains(StatusFlag::C), expected_value);
                }
            }
        }

        Ok(())
    }
}
