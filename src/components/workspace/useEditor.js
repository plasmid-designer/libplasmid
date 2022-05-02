import { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { useRecoilState } from 'recoil'
import { defer } from 'lodash'

import { sequenceState } from '../../state/atoms'

import SequenceDataModel, { SequenceDataCursorModel, SequenceDataSelectionModel } from './SequenceDataModel'

const Bridge = {
    calculateSequenceData: () => invoke('calculate_sequence_data'),
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
    resetSelection: () => invoke('reset_selection'),
    expandSelectionLeft: () => invoke('expand_selection_left'),
    expandSelectionRight: () => invoke('expand_selection_right'),
}

const iupacChars = "ACGTWSMKRYBVDHN-"

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

    const [localSelection, setLocalSelection] = useState({ isSelecting: false, start: 0, end: 0 })
    const [, setSequence] = useRecoilState(sequenceState)

    useEffect(() => {
        updateSequence()
    }, [])

    useEffect(() => {
        setSequence(sequenceModel.nucleotideSequence)
    }, [setSequence, sequenceModel])

    useEffect(() => {
        const updateSelection = async () => {
            if (localSelection.start === 0 && localSelection.end === 0) {
                return
            } else if (localSelection.start === localSelection.end) {
                await Bridge.resetSelection()
            } else {
                await Bridge.setSelection(localSelection.start, localSelection.end)
            }
            await updateSequence()
        }
        updateSelection()
    }, [localSelection])

    /**
     * @param {KeyboardEvent} e
     */
    const handleKeyDown = useCallback(async e => {
        e.preventDefault()

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
                if (ctrl && upperKey === 'V') {
                    const text = await navigator.clipboard.readText()
                    await Bridge.insertAll(text)
                    return
                }
                if (iupacChars.includes(upperKey)) {
                    await Bridge.insert(upperKey)
                }
        }

        return true
    }, [])

    /**
     * @param {MouseEvent} e
     */
    const handleMouseDown = useCallback(async e => {
        e.preventDefault()

        const findIndex = (currentTarget, depth = 0) => {
            if (depth === 3) return null
            if (currentTarget.dataset.index) return parseInt(currentTarget.dataset.index)
            return findIndex(currentTarget.parentElement, depth + 1)
        }

        const index = findIndex(e.target)

        if (index !== null) await Bridge.moveCursorTo(index)
        else await Bridge.moveCursorToEnd()

        setLocalSelection({ start: index, end: index, isSelecting: true })

        return true
    }, [])

    const handleSelectionUpdate = useCallback(async (e, isEndMotion) => {
        const findIndex = (currentTarget, depth = 0) => {
            if (depth === 3) return null
            if (currentTarget.dataset.index) return parseInt(currentTarget.dataset.index)
            return findIndex(currentTarget.parentElement, depth + 1)
        }

        const index = findIndex(e.target)

        if (index !== null && localSelection.start !== index) {
            setLocalSelection(({ start }) => ({ start, end: index, isSelecting: !isEndMotion }))
        }
    }, [localSelection])

    /**
     * @param {MouseEvent} e
     */
    const handleMouseMove = useCallback(async e => {
        e.preventDefault()

        if (localSelection.isSelecting) {
            await handleSelectionUpdate(e, false)
        }

        return false
    }, [handleSelectionUpdate, localSelection])

    /**
     * @param {MouseEvent} e
     */
    const handleMouseUp = useCallback(async e => {
        e.preventDefault()

        await handleSelectionUpdate(e, true)

        setLocalSelection({ start: 0, end: 0, isSelecting: false })
        defer(() => {
            setLocalSelection({ start: 0, end: 0, isSelecting: false })
        })

        return false
    }, [handleSelectionUpdate])

    const updateSequence = async () => {
        const data = await Bridge.calculateSequenceData()
        if (data.sequence) {
            setSequenceModel(new SequenceDataModel(data))
        }
        setCursorModel(new SequenceDataCursorModel(data?.cursor))
        setSelectionModel(new SequenceDataSelectionModel(data?.selection))
    }

    const wrapUpdatingAsync = useCallback(fn => async (...data) => {
        if (await fn(...data)) {
            await updateSequence()
        }
    }, [])

    return {
        cursor: cursorModel,
        sequence: sequenceModel,
        selection: selectionModel,
        handlers: {
            handleKeyDown: wrapUpdatingAsync(handleKeyDown),
            handleMouseDown: wrapUpdatingAsync(handleMouseDown),
            handleMouseMove: wrapUpdatingAsync(handleMouseMove),
            handleMouseUp: wrapUpdatingAsync(handleMouseUp),
        }
    }
}

export default useEditor
