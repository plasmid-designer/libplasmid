import { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useRecoilState } from 'recoil'

import { sequenceState } from '../../state/atoms'

import SequenceDataModel, { SequenceDataCursorModel, SequenceDataSelectionModel } from './SequenceDataModel'
import useSelection from './useSelection'

const Bridge = {
    calculateSequenceData: force => invoke('calculate_sequence_data', { force }),
    insert: letter => invoke('sequence_insert', { letter }),
    insertAll: text => invoke('sequence_insert_all', { text }),
    delete: () => invoke('sequence_delete'),
    deleteNext: () => invoke('sequence_delete_next'),
    moveCursorTo: index => invoke('move_cursor', { index }),
    moveCursorLeft: () => invoke('move_cursor_left'),
    moveCursorRight: () => invoke('move_cursor_right'),
    moveCursorToCodonStart: () => invoke('move_cursor_to_codon_start'),
    moveCursorToCodonEnd: () => invoke('move_cursor_to_codon_end'),
    moveCursorToStart: () => invoke('move_cursor_to_start'),
    moveCursorToEnd: () => invoke('move_cursor_to_end'),
    setSelection: (start, end) => invoke('set_selection', { start, end }),
    selectAll: () => invoke('set_selection_all'),
    resetSelection: () => invoke('reset_selection'),
    expandSelectionLeft: () => invoke('expand_selection_left'),
    expandSelectionRight: () => invoke('expand_selection_right'),
    getSelectedSequence: () => invoke('get_selected_sequence'),
}

const iupacChars = "ACGTWSMKRYBVDHN-"

/**
 * @param {HTMLElement} currentTarget
 */
 const findIndex = (currentTarget) => {
    if (currentTarget.dataset.index) return parseInt(currentTarget.dataset.index)
    if (currentTarget.parentElement.dataset.index) return parseInt(currentTarget.parentElement.dataset.index)
    return null
}

/**
 * @returns {{
 *  cursor: import('./SequenceDataModel').SequenceDataCursorModel,
 *  sequence: import('./SequenceDataModel').default,
 *  selection: import('./SequenceDataModel').SequenceDataSelectionModel,
 * }}
 */
const useEditor = () => {
    const [sequenceModel, setSequenceModel] = useState(new SequenceDataModel())
    const [cursorModel, setCursorModel] = useState(new SequenceDataCursorModel())
    const [selectionModel, setSelectionModel] = useState(new SequenceDataSelectionModel())

    const [, setSequence] = useRecoilState(sequenceState)

    const {
        isSelecting,
        selection,
        startSelection,
        updateSelection,
        endSelection,
    } = useSelection()

    useEffect(() => {
        updateSequence(true)
    }, [])

    useEffect(() => {
        setSequence(sequenceModel.nucleotideSequence)
    }, [setSequence, sequenceModel])

    useEffect(() => {
        const updateBackendSelection = async () => {
            if (selection.start === 0 && selection.end === 0) {
                return
            } else if (selection.start === selection.end) {
                await Bridge.resetSelection()
            } else {
                await Bridge.setSelection(selection.start, selection.end)
            }
            await updateSequence()
        }
        updateBackendSelection()
    }, [selection])

    /**
     * @param {KeyboardEvent} e
     */
    const handleKeyDown = useCallback(async e => {
        e.preventDefault()
        e.stopPropagation()

        const upperKey = e.key.toUpperCase()
        const ctrl = e.ctrlKey
        const shift = e.shiftKey

        switch (e.code) {
            case 'Backspace':
                await Bridge.delete()
                break
            case 'Delete':
                await Bridge.deleteNext()
                break
            case 'ArrowLeft':
                if (ctrl) await Bridge.moveCursorToCodonStart()
                else if (shift) await Bridge.expandSelectionLeft()
                else await Bridge.moveCursorLeft()
                break
            case 'ArrowRight':
                if (ctrl) await Bridge.moveCursorToCodonEnd()
                else if (shift) await Bridge.expandSelectionRight()
                else await Bridge.moveCursorRight()
                break
            default:
                if (ctrl && upperKey === 'C') {
                    await navigator.clipboard.writeText(await Bridge.getSelectedSequence())
                    return true
                } else if (ctrl && upperKey === 'V') {
                    const text = await navigator.clipboard.readText()
                    await Bridge.insertAll(text)
                    return true
                } else if (ctrl && upperKey === 'A') {
                    await Bridge.selectAll()
                    return true
                } else if (ctrl && upperKey === 'X') {
                    await navigator.clipboard.writeText(await Bridge.getSelectedSequence())
                    await Bridge.delete()
                }
                if (iupacChars.includes(upperKey)) {
                    await Bridge.insert(upperKey)
                }
        }

        return true
    }, [])

    const handleMouseEvent = useCallback(async e => {
        e.preventDefault()
        e.stopPropagation()

        const index = findIndex(e.target)

        switch (e.type) {
            case 'mousedown':
                if (index !== null) {
                    startSelection(index)
                    await Bridge.moveCursorTo(index)
                } else {
                    await Bridge.moveCursorToEnd()
                }
                return true
            case 'mousemove':
                if (isSelecting && index !== null) {
                    updateSelection(index)
                }
                return false
            case 'mouseup':
                endSelection(index ?? selection.end)
                return false
            default:
                return false
        }
    }, [isSelecting, startSelection, updateSelection, endSelection, selection])

    const updateSequence = async (force = false) => {
        const data = await Bridge.calculateSequenceData(force)
        if (data.sequence) {
            setSequenceModel(new SequenceDataModel(data))
        }
        setCursorModel(new SequenceDataCursorModel(data?.cursor))
        setSelectionModel(new SequenceDataSelectionModel(data?.selection))
    }

    const wrapUpdatingAsync = fn => async (...data) => {
        if (await fn(...data)) {
            await updateSequence()
        }
    }

    return {
        cursor: cursorModel,
        sequence: sequenceModel,
        selection: selectionModel,
        handlers: {
            handleKeyDown: wrapUpdatingAsync(handleKeyDown),
            handleMouseEvent: wrapUpdatingAsync(handleMouseEvent),
        }
    }
}

export default useEditor
