import { useState, useCallback, useMemo } from 'react'

const useSelection = () => {
    const [isSelecting, setIsSelecting] = useState(false)
    const [selection, setSelection] = useState({start: 0, end: 0})

    const startSelection = useCallback((start) => {
        if (start === selection.start) return
        setIsSelecting(true)
        setSelection({ start, end: start })
    }, [selection.start])

    const updateSelection = useCallback((end) => {
        if (!isSelecting || end === selection.end) return
        setSelection({ start: selection.start, end })
    }, [selection, isSelecting])

    const endSelection = useCallback((end) => {
        if (!isSelecting) return
        setSelection({ start: selection.start, end })
        setIsSelecting(false)
    }, [selection.start, isSelecting])

    const resetSelection = useMemo(() => () => {
        setIsSelecting(false)
    }, [])

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
