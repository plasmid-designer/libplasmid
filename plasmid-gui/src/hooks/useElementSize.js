import { useCallback, useState, useLayoutEffect, useEffect } from 'react'

export const useElementSize = node => {
    const [ref, setRef] = useState(null)
    const [size, setSize] = useState({width: 0, height: 0})

    const handleResize = useCallback(() => {
        setSize({width: ref?.offsetWidth ?? 0, height: ref?.offsetHeight ?? 0})
    }, [ref?.offsetWidth, ref?.offsetHeight])

    useEffect(() => {
        window.addEventListener('resize', handleResize)
        return () => window.removeEventListener('resize', handleResize)
    }, [handleResize])

    useLayoutEffect(() => {
        handleResize()
    }, [handleResize, ref?.offsetWidth, ref?.offsetHeight])

    return [setRef, size]
}
