const { resolve } = require('path')
const fs = require('fs')
const marked = require('marked')

const root = resolve(__dirname, './documentations')

const instructions = fs.readdirSync(root).sort().map(e => e.replace('.md', ''))

let string = ''

instructions.forEach(instruction => {
  const document = fs.readFileSync(resolve(root, `${instruction}.md`), { encoding: 'utf8'})
  const token = marked.lexer(document)
  const implementationidx = token.findIndex(e => e.type === 'heading' && e.depth === 2 && e.text === 'Implementation')
  const implementation = token[implementationidx + 1];
  console.log(implementation)
  string += `#[allow(unused_macros)]
macro_rules! ${instruction.toLowerCase()} {
  ($self:expr, $memory:expr) => {
    ${implementation.text}
  }
}

`
})

const targetFile = resolve(__dirname, '../src/cpu/instructions.rs')

fs.writeFileSync(targetFile, string)
