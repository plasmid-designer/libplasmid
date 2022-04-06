#!/usr/bin/env node

import fs from 'fs/promises'

const detectMode = seq => seq.includes('(') ? 'A' : 'M'
const normalizeName = name => name.replace(/-HF®|\./g, '').replace(/-/g, '_')
const splitSequence = seq => seq.replace(/\(.*?\)/g, '').split('/')

const compile = (seq, name) => {
    const mode = detectMode(seq)
    const [seq1, seq2] = splitSequence(seq)
    const names = [...new Set(name.split(' ').map(normalizeName))]
    if (mode === 'A') {
        // Post processing
        const l = seq.replace(/[A-Z]+/g, '').replace(/^((\(.+?\)).*?)$/gm, '$2')
        const r = seq.replace(/[A-Z]+/g, '').replace(/^(\(.+?\).*?(\(.+?\)))$/gm, '$2')
    }
    return {
        mode,
        names,
        seq1: seq1.split('').join(','),
        seq2: (seq2 ?? '').split('').join(','),
    }
}

const main = async () => {
    const contents = await fs.readFile('data/restriction_enzymes.txt', 'utf8')
    const lines = contents.split('\n')
    for (const line of lines) {
        const seq = line.split(' ')[0]
        // Ignore non-palindromic enzymes for now
        if (seq.includes('(')) continue;
        const names = line.substring(seq.length).trim();
        const data = compile(seq, names)
        if (data.mode === 'A') continue; // Ignore this mode for now
        for (const name of data.names) {
            console.log(`define_enzyme!(${data.mode}; ${name}: [${data.seq1}], [${data.seq2}]),`)
        }
    }
}

await main()