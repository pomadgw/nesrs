extern crate serde;
extern crate serde_yaml;

mod test_utils;

#[path = "instr_test/tests.rs"]
mod instr_test;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_tests {
    use crate::test_utils::*;
    use nesrs::cpu::types::*;
    use nesrs::cpu::*;
    use nesrs::memory::*;
    use serde::{Deserialize, Serialize};
    use serde_yaml;
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct InitMemory {
        start: usize,
        values: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CheckFlag {
        n: Option<bool>,
        c: Option<bool>,
        z: Option<bool>,
        i: Option<bool>,
        v: Option<bool>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Expected {
        read_from: Option<String>,
        target_address: Option<usize>,
        value: Option<u16>,
        cycles: Option<u32>,
        check_flag: Option<CheckFlag>,
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
        description: String,
        code: Vec<u8>,
        expected: Expected,
        cpu: Option<CPUMock>,
        init_memories: Option<Vec<InitMemory>>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct OpcodeCase {
        cases: Vec<Case>,
    }

    #[test]
    fn test() -> std::io::Result<()> {
        let mut file = File::open("./tests/testlist.yml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let deserialized_map: BTreeMap<String, OpcodeCase> =
            serde_yaml::from_str(&contents).unwrap();
        for (key, value) in deserialized_map.iter() {
            println!("[TEST] Opcode {}", key);

            for case in &value.cases {
                println!("[TEST]   > testing: {}", case.description);

                let mut cpu = CPU::new();

                let mut memory = RAM::new();
                set_reset!(memory, 0x8001);

                cpu.reset();
                loop_cpu!(cpu, memory);

                if let Some(regs) = &case.cpu {
                    cpu.regs.a = regs.a.unwrap_or(0);
                    cpu.regs.x = regs.x.unwrap_or(0);
                    cpu.regs.y = regs.y.unwrap_or(0);
                    cpu.regs.sp = regs.sp.unwrap_or(0);
                    if let Some(p) = regs.p {
                        cpu.regs.p.set_from_byte(p);
                    }
                }

                if let Some(configs) = &case.init_memories {
                    for config in configs {
                        let mut offset = 0;

                        for v in &config.values {
                            memory.write(config.start + offset, *v);
                            offset += 1;
                        }
                    }
                }

                let prev_cycle = cpu.total_cycles;
                set_ram_from_vec!(memory, 0x8001, &case.code);

                loop_cpu!(cpu, memory);

                if let Some(read_from) = &case.expected.read_from {
                    let expected_value = case.expected.value.unwrap();

                    match &read_from[..] {
                        "a" => {
                            assert_eq!(
                                cpu.regs.a, expected_value as u8,
                                "Register A EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value, cpu.regs.a
                            );
                        }
                        "x" => {
                            assert_eq!(
                                cpu.regs.x, expected_value as u8,
                                "Register X EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value, cpu.regs.x
                            );
                        }
                        "y" => {
                            assert_eq!(
                                cpu.regs.y, expected_value as u8,
                                "Register Y EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value, cpu.regs.y
                            );
                        }
                        "sp" => {
                            assert_eq!(
                                cpu.regs.sp, expected_value as u8,
                                "Stack position EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value, cpu.regs.a
                            );
                        }
                        "p" => {
                            assert_eq!(
                                cpu.regs.p.bits(),
                                expected_value as u8,
                                "Status regiister EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value,
                                cpu.regs.a
                            );
                        }
                        "pc" => {
                            assert_eq!(
                                cpu.regs.pc, expected_value,
                                "PC EXPECTED: {:02X} RESULT: {:02X}",
                                expected_value, cpu.regs.a
                            );
                        }
                        "address" => {
                            let result =
                                memory.read(case.expected.target_address.unwrap_or(0), false);
                            assert_eq!(
                                result,
                                expected_value as u8,
                                "Memory content {:04X} EXPECTED: {:02X} RESULT: {:02X}",
                                case.expected.target_address.unwrap_or(0),
                                expected_value,
                                result
                            );
                        }
                        _ => {}
                    }
                }

                if let Some(expected_cycle) = case.expected.cycles {
                    assert_eq!(cpu.total_cycles - prev_cycle, expected_cycle);
                }

                if let Some(flag_check) = &case.expected.check_flag {
                    if let Some(expected_value) = flag_check.n {
                        println!("[TEST]      > Test N flag");
                        assert_eq!(
                            cpu.regs.p.contains(StatusFlag::N),
                            expected_value,
                            "N flags EXPECTED: {} RESULT: {}",
                            expected_value,
                            cpu.regs.p.contains(StatusFlag::N)
                        );
                    }

                    if let Some(expected_value) = flag_check.z {
                        println!("[TEST]      > Test Z flag");
                        assert_eq!(
                            cpu.regs.p.contains(StatusFlag::Z),
                            expected_value,
                            "Z flags EXPECTED: {} RESULT: {}",
                            expected_value,
                            cpu.regs.p.contains(StatusFlag::Z)
                        );
                    }

                    if let Some(expected_value) = flag_check.c {
                        println!("[TEST]      > Test C flag");
                        assert_eq!(
                            cpu.regs.p.contains(StatusFlag::C),
                            expected_value,
                            "C flags EXPECTED: {} RESULT: {}",
                            expected_value,
                            cpu.regs.p.contains(StatusFlag::C)
                        );
                    }

                    if let Some(expected_value) = flag_check.i {
                        println!("[TEST]      > Test I flag");
                        assert_eq!(
                            cpu.regs.p.contains(StatusFlag::I),
                            expected_value,
                            "I flags EXPECTED: {} RESULT: {}",
                            expected_value,
                            cpu.regs.p.contains(StatusFlag::I)
                        );
                    }

                    if let Some(expected_value) = flag_check.v {
                        println!("[TEST]      > Test V flag");
                        assert_eq!(
                            cpu.regs.p.contains(StatusFlag::V),
                            expected_value,
                            "V flags EXPECTED: {} RESULT: {}",
                            expected_value,
                            cpu.regs.p.contains(StatusFlag::V)
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
