import { useState, useEffect, useCallback } from 'react'
import { useRecoilState } from 'recoil'
import { invoke } from '@tauri-apps/api/tauri'

import { sequenceState } from '../../state/atoms'

import SequenceDataModel from './SequenceDataModel'

const Bridge = {
    calculateSequenceData: () => invoke('calculate_sequence_data'),
    insert: letter => invoke('sequence_insert', { letter }),
    insertAll: text => invoke('sequence_insert_all', { text }),
    delete: () => invoke('sequence_delete'),
    moveCursorTo: index => invoke('move_cursor', { index }),
    moveCursorLeft: () => invoke('move_cursor_left'),
    moveCursorRight: () => invoke('move_cursor_right'),
    moveCursorToCodonStart: () => invoke('move_cursor_to_codon_start'),
    moveCursorToCodonEnd: () => invoke('move_cursor_to_codon_end'),
    moveCursorToStart: () => invoke('move_cursor_to_start'),
    moveCursorToEnd: () => invoke('move_cursor_to_end'),
}

const iupacChars = "ACGTWSMKRYBVDHN-"

const useEditor = () => {
    const [data, setData] = useState(new SequenceDataModel())
    const [, setSequence] = useRecoilState(sequenceState)

    const handleKeyDown = async e => {
        e.preventDefault()
        const upperKey = e.key.toUpperCase()
        const ctrl = e.ctrlKey
        switch (e.code) {
            case 'Backspace':
            case 'Delete':
                await Bridge.delete()
                break;
            case 'ArrowLeft':
                if (ctrl) await Bridge.moveCursorToCodonStart()
                else await Bridge.moveCursorLeft()
                break;
            case 'ArrowRight':
                if (ctrl) await Bridge.moveCursorToCodonEnd()
                else await Bridge.moveCursorRight()
                break;
            default:
                if (ctrl && upperKey === 'V') {
                    const text = await navigator.clipboard.readText()
                    await Bridge.insertAll(text)
                    return
                }
                if (iupacChars.includes(upperKey)) {
                    await Bridge.insert(upperKey)
                }
        }
    }

    const handleMouseDown = async e => {
        e.preventDefault()
        const findIndex = (currentTarget, depth = 0) => {
            if (depth === 3) return null
            if (currentTarget.dataset.index) return parseInt(currentTarget.dataset.index)
            return findIndex(currentTarget.parentElement, depth + 1)
        }
        const index = findIndex(e.target)
        if (index !== null) await Bridge.moveCursorTo(index)
        else await Bridge.moveCursorToEnd()
    }

    const updateSequence = async () => {
        const data = await Bridge.calculateSequenceData()
        const model = new SequenceDataModel(data)
        setData(model)
        setSequence(model.nucleotideSequence())
    }

    const wrapUpdatingAsync = fn => async (...data) => {
        await fn(...data)
        await updateSequence()
    }

    return {
        data,
        handlers: {
            handleKeyDown: wrapUpdatingAsync(handleKeyDown),
            handleMouseDown: wrapUpdatingAsync(handleMouseDown),
        }
    }
}

export default useEditor
