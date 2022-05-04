import { useState, useCallback, useMemo } from 'react'
import { unstable_batchedUpdates, flushSync } from 'react-dom'

const useSelection = () => {
    const [isSelecting, setIsSelecting] = useState(false)
    const [selection, setSelection] = useState({start: 0, end: 0})

    const startSelection = useCallback((start) => {
        if (start === selection.start) return
        flushSync(() => setIsSelecting(true))
        setSelection({ start, end: 0 })
    }, [selection.start])

    const updateSelection = useCallback((end) => {
        if (!isSelecting || end === selection.end) return
        setSelection({ start: selection.start, end })
    }, [selection, isSelecting])

    const endSelection = useCallback((end) => {
        if (!isSelecting) return
        setSelection({ start: selection.start, end })
        flushSync(() => setIsSelecting(false))
    }, [selection.start, isSelecting])

    const resetSelection = useMemo(() => () => {
        flushSync(() => setIsSelecting(false))
    }, [])

    console.log(`Selecting: ${isSelecting}; Start: ${selection.start}; End: ${selection.end}`)

    return {
        isSelecting,
        selection,
        startSelection,
        updateSelection,
        endSelection,
        resetSelection,
    }
}

export default useSelection
