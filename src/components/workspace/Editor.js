import { memo, useMemo, useRef, useEffect, useCallback, useState } from 'react'
import { useRecoilValue } from 'recoil'
import { compact } from 'lodash'
import styled from 'styled-components'

import useEditor from './useEditor'
import EditorToolbar from './EditorToolbar'

import ColorUtil from '../../util/ColorUtil'
import { editorHintState, editorRendererState } from '../../state/atoms'

import CoreRendererV1 from './renderers/CoreRendererV1'
import NextRenderer from './renderers/NextRenderer'

const Editor = ({ className }) => {
    const editorRef = useRef()
    const [renderCursor, setRenderCursor] = useState(false)
    const rendererName = useRecoilValue(editorRendererState)

    const {
        cursor,
        sequence,
        selection,
        handlers: {
            handleKeyDown,
            handleMouseEvent,
        },
    } = useEditor()

    useEffect(() => {
        editorRef.current?.focus()
    }, [])

    const handleFocusChange = useCallback((showCursor, refocus = false) => () => {
        setRenderCursor(showCursor)
        if (refocus) { editorRef.current?.focus() }
    }, [])

    const RendererComponent = useMemo(() => {
        switch (rendererName) {
            case 'core/v1': return CoreRendererV1
            case 'next': return NextRenderer
            default: return CoreRendererV1
        }
    }, [rendererName])

    return (
        <div className={className}>
            <EditorToolbar />
            <div
                ref={editorRef}
                className="editor"
                onKeyDown={handleKeyDown}
                onMouseDown={handleMouseEvent}
                onMouseMove={handleMouseEvent}
                onMouseUp={handleMouseEvent}
                onFocus={handleFocusChange(true)}
                onClick={handleFocusChange(true, true)}
                onBlur={handleFocusChange(false)}
                tabIndex={0}
            >
                <RendererComponent sequence={sequence} cursor={cursor} selection={selection} showCursor={renderCursor} />
            </div>
        </div>
    )
}

export default styled(Editor)`
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
    flex-grow: 1;

    .editor {
        width: 100%;
        height: 100%;
        overflow: auto;
        padding: .5rem;

        &:focus {
            outline: none;
        }
    }
`
