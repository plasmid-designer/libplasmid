import { memo, useState, useRef, useMemo, useEffect, useReducer } from 'react'
import useComponentSize from '@rehooks/component-size'
import styled from 'styled-components'

const getPanelWidth = (panelOptions, fullWidth, calculatedWidth) => {
    return (
        Math.min(calculatedWidth, fullWidth)
        ?? panelOptions?.startWidth
        ?? (
            ('startRatio' in panelOptions)
            ? fullWidth * panelOptions.startRatio
            : fullWidth / 2
        )
    )
}

const getPanelStyle = (panelOptions, fullWidth, calculatedWidth) => {
    const actualWidth = getPanelWidth(panelOptions, fullWidth, calculatedWidth)
    const obj = {
        width: `${actualWidth}px`,
        height: '100%',
    }
    if (panelOptions?.minWidth) {
        obj.minWidth = `${panelOptions.minWidth}px`
    }
    return obj
}

const ResizableSplitPanel = ({ className, leftComponent, rightComponent, options, ...rest }) => {
    const ref = useRef(null)
    const { width } = useComponentSize(ref)

    const [isInitialized, markAsInitialized] = useReducer(() => true, false)
    const [isResizing, setIsResizing] = useState(false)
    const [initialPos, setInitialPos] = useState(null)
    const [leftPanelWidth, setLeftPanelWidth] = useState(0)
    const rightPanelWidth = width - leftPanelWidth

    const leftPanelStyle = getPanelStyle(options?.left, width, leftPanelWidth)
    const rightPanelStyle = getPanelStyle(options?.right, width, rightPanelWidth)

    useEffect(() => {
        if (!isInitialized && width !== 0) {
            setLeftPanelWidth(getPanelWidth(options?.left, width, null))
            markAsInitialized()
        } else if (!isResizing) {
            setLeftPanelWidth(getPanelWidth(options?.left, width - options?.left?.minWidth ?? 0, leftPanelWidth))
        }
    }, [width])

    /**
     * @param {MouseEvent} e
     */
    const handleMouseEvent = e => {
        if (e.target.parentElement !== ref.current) {
            return
        }

        e.preventDefault()

        const x = e.screenX
        const calculateWidth = diff => {
            return Math.max(
                options.left.minWidth ?? 0,
                Math.min(
                    width * 0.75,
                    diff + leftPanelWidth
                )
            )
        }

        switch (e.type) {
            case 'mousedown': {
                setInitialPos(x)
                setIsResizing(true)
                return
            }
            case 'mousemove': {
                if (!isResizing) return
                const diff = x - initialPos
                setInitialPos(x)
                setLeftPanelWidth(calculateWidth(diff))
                return
            }
            case 'mouseup': {
                if (!isResizing) return
                const diff = x - initialPos
                setLeftPanelWidth(calculateWidth(diff))
                setIsResizing(false)
                return
            }
        }
    }

    return (
        <div
            ref={ref}
            className={className}
            onMouseMove={handleMouseEvent}
            onMouseUp={handleMouseEvent}
            {...rest}
        >
            <div className="panel">
                <div className="panel__inner" style={{...leftPanelStyle, pointerEvents: isResizing ? 'none' : 'auto'}}>
                    {leftComponent}
                </div>
            </div>
            <div id="resizeHandle" className="resizeHandle" onMouseDown={handleMouseEvent} />
            <div className="panel">
                <div className="panel__inner" style={{...rightPanelStyle, pointerEvents: isResizing ? 'none' : 'auto'}}>
                    {rightComponent}
                </div>
            </div>
        </div>
    )
}

export default styled(ResizableSplitPanel)`
    display: flex;
    flex-flow: row nowrap;
    justify-content: space-between;
    width: 100%;
    height: 100%;

    & > .panel {
        width: 100%;
        height: 100%;
        user-select: none;
    }

    & .resizeHandle {
        width: .5rem;
        min-width: .5rem;
        max-width: .5rem;
        height: 100%;
        background: hsl(0,0%,20%);
        cursor: col-resize;
    }
`
