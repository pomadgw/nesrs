const { resolve } = require('path')
const fs = require('fs')
const marked = require('marked')

const root = resolve(__dirname, './documentations')

const instructions = fs.readdirSync(root).sort().map(e => e.replace('.md', ''))

let string = ''


const templateClock = (result) => `
use crate::cpu::*;
use crate::Memory;

impl CPU {
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        self.init_opcode(memory);

        match self.current_opcode {
            ${result}
            _ => {
              self.steps = 1;
            }
        }

        self.steps -= 1;
        self.cycles += 1;
    }
}
`

const implementedAddresingModes = {
  'Implied': 'imp',
  'Immediate': 'imm',
  'Zero Page': 'zp0',
  'Zero Page,X': 'zpx',
  'Zero Page,Y': 'zpy',
  'Absolute': 'abs',
  'Absolute,X': 'abx',
  'Absolute,Y': 'aby',
  'Indirect,X': 'izx',
  'Indirect,Y': 'izy',
  // 'Indirect': 'ind',
  // 'Accumulator': 'acc',
  // 'Relative': 'rel',
}

const opcodes = []

instructions.forEach(instruction => {
  const document = fs.readFileSync(resolve(root, `${instruction}.md`), { encoding: 'utf8'})
  const token = marked.lexer(document)
  const addressingidx = token.findIndex(e => e.type === 'heading' && e.depth === 2 && e.text === 'Addresing Modes')
  const addressing = token[addressingidx + 1];

  if (addressing?.type === 'table') {
    const mapOpcode = addressing.cells.map(cell => ({
      opcode: cell[2].substr(1, 3),
      mode: implementedAddresingModes[cell[0]],
      cycles: Number(cell[4].replace('+', '')),
      handleCrossPage: cell[4].indexOf('+') != -1
    }))

    mapOpcode.forEach(({ opcode, cycles, mode }) => {
      opcodes.push(`0x${opcode.toLowerCase()} => {
        set_instruction!(self, ${cycles}, {
          ${mode}!(self, memory);
          ${instruction}!(self, memory);
        });
      },`)
    })
  }

  const implementationidx = token.findIndex(e => e.type === 'heading' && e.depth === 2 && e.text === 'Implementation')
  const implementation = token[implementationidx + 1];

  string += `#[allow(unused_macros)]
macro_rules! ${instruction.toLowerCase()} {
  ($self:expr, $memory:expr) => {
    ${implementation.text}
  }
}

`
})

const targetFile = resolve(__dirname, '../src/cpu/instructions.rs')
const targetClockFile = resolve(__dirname, '../src/cpu/clock.rs')

opcodes.sort()

fs.writeFileSync(targetFile, string)
fs.writeFileSync(targetClockFile, templateClock(opcodes.join('\n')))
