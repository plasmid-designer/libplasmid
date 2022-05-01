import { useState, useEffect, useCallback } from 'react'

import { invoke } from '@tauri-apps/api/tauri'
import { useRecoilState } from 'recoil'
import { sequenceState } from '../../state/atoms'

const Bridge = {
    calculateSequenceData: sequence => invoke('calculate_sequence_data', { sequence }),
}

const iupacChars = "ACGTWSMKRYBVDHN-"

const useEditor = () => {
    const [cursorIndex, setCursorIndex] = useState(0)
    const [sequence, setSequence] = useRecoilState(sequenceState)
    const [finalSequence, setFinalSequence] = useState([])

    const handleKeyDown = useCallback(async e => {
        const upperKey = e.key.toUpperCase()
        const ctrl = e.ctrlKey
        switch (e.code) {
            case 'Backspace':
            case 'Delete':
                if (cursorIndex === 0) return
                if (cursorIndex === sequence.length) {
                    setSequence(sequence.slice(0, -1))
                    setCursorIndex(index => index - 1)
                } else {
                    setSequence(seq => [...seq.slice(0, cursorIndex - 1), ...seq.slice(cursorIndex)])
                    setCursorIndex(index => index - 1)
                }
                break;
            case 'ArrowLeft':
                if (ctrl) setCursorIndex(index => Math.max(0, index - (index % 3 === 0 ? 3 : index % 3)))
                else setCursorIndex(index => Math.max(0, index - 1))
                break;
            case 'ArrowRight':
                if (ctrl) setCursorIndex(index => Math.min(index + (3 - index % 3), sequence.length))
                else setCursorIndex(index => Math.min(index + 1, sequence.length))
                break;
            default:
                if (ctrl && ['v', 'V'].includes(e.key)) {
                    const text = await navigator.clipboard.readText()
                    const nucleotides = [...text].filter(char => iupacChars.includes(char))
                    setSequence(seq => [...seq.slice(0, cursorIndex), ...nucleotides, ...seq.slice(cursorIndex)])
                    setCursorIndex(index => index + nucleotides.length)
                    return
                }
                if (iupacChars.includes(upperKey)) {
                    if (cursorIndex === sequence.length - 1) {
                        setSequence(seq => [...seq, upperKey])
                    } else {
                        setSequence(seq => [...seq.slice(0, cursorIndex), upperKey, ...seq.slice(cursorIndex)])
                    }
                    setCursorIndex(index => index + 1)
                }
        }
    }, [cursorIndex, sequence, setSequence])

    const handleMouseDown = useCallback(e => {
        const findIndex = (currentTarget, depth = 0) => {
            if (depth === 3) return null
            if (currentTarget.dataset.index) return parseInt(currentTarget.dataset.index)
            return findIndex(currentTarget.parentElement, depth + 1)
        }
        const index = findIndex(e.target)
        if (index !== null) setCursorIndex(index)
        else setCursorIndex(sequence.length)
    }, [sequence.length])

    useEffect(() => {
        const calculateSequence = async () => {
            setFinalSequence(await Bridge.calculateSequenceData(sequence))
        }
        calculateSequence()
    }, [sequence])

    return {
        data: {
            sequence: finalSequence,
            cursorIndex,
            cursorEndIndex: sequence.length,
            cursorAtEnd: cursorIndex === sequence.length,
        },
        handlers: {
            handleKeyDown,
            handleMouseDown,
        }
    }
}

export default useEditor
