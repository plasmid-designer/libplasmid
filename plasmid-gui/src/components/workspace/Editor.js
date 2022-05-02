import { useRef, useEffect, useState } from 'react'
import { useRecoilValue } from 'recoil'
import { compact } from 'lodash'
import styled from 'styled-components'

import useEditor from './useEditor'
import EditorToolbar from './EditorToolbar'

import ColorUtil from '../../util/ColorUtil'
import { editorHintState } from '../../state/atoms'

const SequenceItem = ({
    codon = null,
    anticodon = null,
    peptide = null,
    codonIndex = 0,
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
    const key = `${isStart ? 'start;' : ''}${isEnd ? 'end;' : ''}${codon}${codonIndex}`
    const index = forceIndex ?? (isStart ? 0 : codonIndex)
    return (
        <div data-index={index} className={className} key={key} data-selected={editorHints.highlightCurrentCodon ? selected : false}>
            <div className="sequence__item__codon">
                {isStart && <>5'</>}
                {isEnd && <>3'</>}
                {codon && codon.map((nucleotide, nucIndex) => (
                    <div
                        data-index={codonIndex + nucIndex}
                        key={`codon;${codonIndex}${nucIndex}`}
                        className="sequence__item__codon__nucleotide_wrapper"
                    >
                        {cursorIndex === codonIndex + nucIndex && renderCursor && (
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
                    {anticodon && anticodon.map((nucleotide, nucIndex) => (
                        <div
                            data-index={codonIndex + nucIndex}
                            key={`anticodon;${codonIndex}${nucIndex}`}
                            className="sequence__item__codon__nucleotide_wrapper"
                        >
                            {cursorIndex === codonIndex + nucIndex && renderCursor && (
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
            { editorHints.showCodonNumbers && (
                <div className="sequence__item__peptide_index">
                    {codon && <>{codonIndex + 1}</>}
                </div>
            )}
            { editorHints.showPeptides && (
                <div className="sequence__item__peptide">
                    {peptide && <span style={{backgroundColor: ColorUtil.getPeptideColor(peptide)}}>{peptide}</span>}
                </div>
            )}
        </div>
    )
}

const Editor = ({ className }) => {
    const editorRef = useRef()
    const [renderCursor, setRenderCursor] = useState(false)

    const {
        data,
        handlers: {
            handleKeyDown,
            handleMouseDown,
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
                onFocus={handleFocusChange(true)}
                onClick={handleFocusChange(true, true)}
                onBlur={handleFocusChange(false)}
                tabIndex={0}
            >
                <div className="sequence">
                    {/* <SequenceItem isStart /> */}
                    {data.items().map(item => (
                        <SequenceItem
                            codon={item.codonLetters()}
                            anticodon={item.anticodonLetters()}
                            peptide={item.peptideLetter()}
                            codonIndex={item.startIndex()}
                            cursorIndex={data.cursorPosition()}
                            renderCursor={renderCursor}
                            selected={renderCursor && data.isItemSelected(item)}
                        />
                    ))}
                    {data.isCursorAtEnd() && <SequenceItem onlyCursor forceIndex={data.bpCount()} renderCursor={renderCursor} />}
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
