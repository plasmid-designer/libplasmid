import { memo, useRef, useEffect, useState } from 'react'
import { useRecoilValue } from 'recoil'
import { compact } from 'lodash'
import styled from 'styled-components'

import useEditor from './useEditor'
import EditorToolbar from './EditorToolbar'

import ColorUtil from '../../util/ColorUtil'
import { editorHintState } from '../../state/atoms'

/**
 * @param {{
 *    item: import('./SequenceDataModel').SequenceDataItemModel,
 *    selection: import('./SequenceDataModel').SequenceDataSelectionModel,
 * }} props
 */
const SequenceItem = ({
    item,
    selection,
    cursorIndex = 0,
    renderCursor = false,
    isStart = false,
    isEnd = false,
    onlyCursor = false,
    selected = false,
    forceIndex = null
}) => {
    const editorHints = useRecoilValue(editorHintState)
    const className = compact([
        'sequence__item',
        isStart ? 'sequence__item--start-marker' : null,
        isEnd ? 'sequence__item--end-marker' : null,
    ]).join(' ')
    const key = `${isStart ? 'start;' : ''}${isEnd ? 'end;' : ''}${item?.startIndex}`
    const index = forceIndex ?? (isStart ? 0 : item?.startIndex)
    return (
        <div data-index={index} className={className} key={key} data-selected={editorHints.highlightCurrentCodon ? selected : false}>
            <div className="sequence__item__codon">
                {isStart && <>5'</>}
                {isEnd && <>3'</>}
                {item && item.codonLetters.map((nucleotide, nucIndex) => (
                    <div
                        data-index={item.startIndex + nucIndex}
                        key={nucIndex}
                        className="sequence__item__codon__nucleotide_wrapper"
                        data-user-selected={selection.contains(item.startIndex + nucIndex)}
                    >
                        {cursorIndex === item.startIndex + nucIndex && renderCursor && (
                            <>&#8203;<div className="cursor">|</div></>
                        )}
                        <span style={{color: ColorUtil.getNucleotideColor(nucleotide)}}>{nucleotide}</span>
                    </div>
                ))}
                {onlyCursor && renderCursor && (
                    <div className="sequence__item__codon__nucleotide_wrapper">
                        &#8203;<div className="cursor">|</div>
                    </div>
                )}
            </div>
            { editorHints.showComplementStrand && (
                <div className="sequence__item__codon sequence__item__codon--anticodon">
                    {isStart && <>3'</>}
                    {isEnd && <>5'</>}
                    {item && item.anticodonLetters.map((nucleotide, nucIndex) => (
                        <div
                            data-index={item.startIndex + nucIndex}
                            key={nucIndex}
                            className="sequence__item__codon__nucleotide_wrapper"
                            data-user-selected={selection.contains(item.startIndex + nucIndex)}
                        >
                            {cursorIndex === item.startIndex + nucIndex && renderCursor && (
                                <>&#8203;<div className="cursor">|</div></>
                            )}
                            <span style={{color: ColorUtil.getNucleotideColor(nucleotide)}}>{nucleotide}</span>
                        </div>
                    ))}
                    {onlyCursor && renderCursor && (
                        <div className="sequence__item__codon__nucleotide_wrapper">
                            &#8203;<div className="cursor">|</div>
                        </div>
                    )}
                </div>
            )}
            { !onlyCursor && editorHints.showCodonNumbers && (
                <div className="sequence__item__peptide_index">
                    {item.codonLetters && item.codonLetters.length > 0 && <>{item.startIndex + 1}</>}
                </div>
            )}
            { !onlyCursor && editorHints.showPeptides && (
                <div className="sequence__item__peptide">
                    {item.peptideLetter && <span style={{backgroundColor: ColorUtil.getPeptideColor(item.peptideLetter)}}>{item.peptideLetter}</span>}
                </div>
            )}
        </div>
    )
}

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
        }
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
                <div className="sequence">
                    {/* <SequenceItem isStart /> */}
                    {sequence.items.map(item => (
                        <SequenceItem
                            item={item}
                            selection={selection}
                            cursorIndex={cursor.cursorPosition}
                            renderCursor={renderCursor}
                            selected={renderCursor && cursor.isItemSelected(item)}
                        />
                    ))}
                    {cursor.isCursorAtEnd() && <SequenceItem onlyCursor forceIndex={sequence.bpCount} renderCursor={renderCursor} />}
                    {/* <SequenceItem isEnd forceIndex={cursorEndIndex} /> */}
                </div>
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

    & .sequence {
        display: flex;
        flex-flow: row wrap;
        font-family: monospace;
        font-size: 14pt;
        cursor: text;

        &__item {
            display: flex;
            flex-direction: column;

            &[data-selected=true] {
                border-radius: .25rem;
                background: hsla(0,0%,0%,.075);
            }

            &__codon {
                display: flex;

                &--anticodon {
                    opacity: .5;
                }

                &__nucleotide_wrapper {
                    display: flex;
                    position: relative;

                    &[data-user-selected=true] {
                        background: hsla(327, 20%, 67%, 0.5);
                    }
                }
            }

            &__peptide_index {
                text-align: center;
                font-size: 10pt;
            }

            &__peptide {
                display: flex;
                width: 100%;
                text-align: center;
                padding: .1rem;

                span {
                    font-size: 10pt;
                    width: 100%;
                    border-radius: .25rem;
                }
            }

            &--start-marker {
                padding-right: .5rem;
            }

            &--end-marker {
                padding-left: .5rem;
            }
        }
    }

    & .cursor {
        position: absolute;
        margin-left: -.4rem;
        user-select: none;
        animation: cursor 1s infinite;
        pointer-events: none;
    }

    @keyframes cursor {
        0% { opacity: 1; }
        49% { opacity: 1; }
        50% { opacity: 0; }
        99% { opacity: 0; }
        100% { opacity: 1; }
    }
`
