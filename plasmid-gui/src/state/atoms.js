import { atom } from 'recoil'

export const sequenceState = atom({
    key: 'sequenceState',
    default: '',
})

export const editorHintState = atom({
    key: 'editorHintState',
    default: {
        showComplementStrand: true,
        showCodonNumbers: true,
        showPeptides: true,
        highlightCurrentCodon: true,
    }
})

export const editorRendererState = atom({
    key: 'editorRendererState',
    default: 'next'
})
