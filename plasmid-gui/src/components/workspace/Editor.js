import { memo, useRef, useEffect, useState } from 'react'
import { useRecoilValue } from 'recoil'
import { compact } from 'lodash'
import styled from 'styled-components'

import useEditor from './useEditor'
import EditorToolbar from './EditorToolbar'

import ColorUtil from '../../util/ColorUtil'
import { editorHintState } from '../../state/atoms'

import CoreRendererV1 from './renderers/CoreRendererV1'

const Editor = ({ className }) => {
    const editorRef = useRef()
    const [renderCursor, setRenderCursor] = useState(false)

    const {
        cursor,
        sequence,
        selection,
        handlers: {
            handleKeyDown,
            handleMouseDown,
            handleMouseMove,
            handleMouseUp,
        },
    } = useEditor()

    useEffect(() => {
        editorRef.current?.focus()
    }, [])

    const handleFocusChange = (showCursor, refocus = false) => () => {
        setRenderCursor(showCursor)
        if (refocus) { editorRef.current?.focus() }
    }

    return (
        <div className={className}>
            <EditorToolbar />
            <div
                ref={editorRef}
                className="editor"
                onKeyDown={handleKeyDown}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onFocus={handleFocusChange(true)}
                onClick={handleFocusChange(true, true)}
                onBlur={handleFocusChange(false)}
                tabIndex={0}
            >
                <CoreRendererV1 sequence={sequence} cursor={cursor} selection={selection} showCursor={renderCursor} />
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
