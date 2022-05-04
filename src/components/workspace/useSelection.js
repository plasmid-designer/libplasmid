import { useState, useCallback, useMemo, useEffect } from 'react'
import { unstable_batchedUpdates, flushSync } from 'react-dom'
import { debounce } from 'lodash'

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

    const  debouncedUpdateSelection = debounce(updateSelection, 20, { maxWait: 200 })

    const endSelection = useCallback((end) => {
        if (!isSelecting) return
        setSelection({ start: selection.start, end })
        setIsSelecting(false)
    }, [selection.start, isSelecting])

    const resetSelection = useMemo(() => () => {
        setIsSelecting(false)
    }, [])

    // useEffect(() => {
    //     console.log(`Selecting: ${isSelecting}; Start: ${selection.start}; End: ${selection.end}`)
    // }, [isSelecting, selection])

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
