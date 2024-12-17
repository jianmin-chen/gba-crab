use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Flags {
    Z: String,
    N: String,
    H: String,
    C: String,
}

impl Flags {
    fn untouched(&self) -> bool {
        if self.Z == "-" && self.N == "-" && self.H == "-" && self.C == "-" {
            return true;
        }
        return false;
    }
}

#[derive(Deserialize, Debug)]
struct Operand {
    name: String,
    increment: Option<bool>,
    decrement: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Instruction {
    mnemonic: String,
    operands: Vec<Operand>,
    bytes: usize,
    flags: Flags,
}

#[derive(Deserialize, Debug)]
struct Instructions {
    unprefixed: HashMap<String, Instruction>,
    cbprefixed: HashMap<String, Instruction>,
}

fn main() {
    let parsed: Instructions = serde_json::from_str(include_str!("instructions.json")).unwrap();

    let mut instructions = parsed.unprefixed.iter();
    let mut cases = File::create("cases.rs").unwrap();
    let mut variants = File::create("variants.rs").unwrap();

    while let Some(entry) = instructions.next() {
        let (opcode, instruction) = entry;

        if instruction.mnemonic.starts_with("ILLEGAL") {
            continue;
        } else if instruction.mnemonic.starts_with("PREFIX") {
            // Special instruction.
            //
            // There is a PREFIX opcode
            // that treats the next byte as an additional opcode,
            // pointing to a whole new table ($CB $xx).
            //
            // This means we get even jankier and
            // introduce a new enum variant later on
            // in a different file!
            write!(&mut variants, "PREFIX(PrefixInstruction),\n").unwrap();
            write!(
                &mut cases,
                "{} => Instruction::PREFIX(self.fetch_prefix()),\n",
                opcode.to_lowercase()
            )
            .unwrap();
            continue;
        }

        let mut mnemonic = instruction.mnemonic.clone();

        if !instruction.flags.untouched() {
            mnemonic.push_str("_ADDR");
        }

        for operand in &instruction.operands {
            mnemonic
                .push_str(format!("_{}", operand.name.to_uppercase().replace("$", "")).as_str());
            if let Some(increment) = operand.increment {
                if increment {
                    mnemonic.push_str("_INC");
                }
            } else if let Some(decrement) = operand.decrement {
                if decrement {
                    mnemonic.push_str("_DEC");
                }
            }
        }

        write!(&mut variants, "{mnemonic}").unwrap();
        write!(
            &mut cases,
            "{} => Instruction::{mnemonic}",
            opcode.to_lowercase()
        )
        .unwrap();

        match instruction.bytes {
            2 => {
                write!(&mut variants, "(u8),\n").unwrap();
                write!(&mut cases, "(self.read()),\n").unwrap();
            }
            3 => {
                write!(&mut variants, "(u16),\n").unwrap();
                write!(&mut cases, "(self.read_word()),\n").unwrap();
            }
            _ => {
                assert!(instruction.bytes < 2);
                write!(&mut variants, ",\n").unwrap();
                write!(&mut cases, ",\n").unwrap();
            }
        }
    }

    instructions = parsed.cbprefixed.iter();
    let mut prefixed_cases = File::create("prefixed_cases.rs").unwrap();
    let mut prefixed_variants = File::create("prefixed_variants.rs").unwrap();

    while let Some(entry) = instructions.next() {
        let (opcode, instruction) = entry;

        let mut mnemonic = instruction.mnemonic.clone();

        for operand in &instruction.operands {
            mnemonic.push_str(format!("_{}", operand.name.to_uppercase()).as_str());
            if let Some(increment) = operand.increment {
                if increment {
                    mnemonic.push_str("_INC");
                }
            } else if let Some(decrement) = operand.decrement {
                if decrement {
                    mnemonic.push_str("_DEC");
                }
            }
        }

        write!(&mut prefixed_variants, "{mnemonic},\n").unwrap();
        write!(
            &mut prefixed_cases,
            "{} => PrefixInstruction::{mnemonic},\n",
            opcode.to_lowercase()
        )
        .unwrap();
    }
}
