import { useRef, useEffect, useState } from 'react'
import { useRecoilValue } from 'recoil'
import { compact } from 'lodash'

import ColorUtil from '../../../util/ColorUtil'
import { editorHintState } from '../../../state/atoms'

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

export const DynamicRenderer = ({
    className,
    sequence,
    cursor,
    selection,
    showCursor,
}) => {
    return (
        <div className="sequence">
            {/* <SequenceItem isStart /> */}
            {sequence.items.map(item => (
                <SequenceItem
                    item={item}
                    selection={selection}
                    cursorIndex={cursor.cursorPosition}
                    renderCursor={showCursor}
                    selected={showCursor && cursor.isItemSelected(item)}
                />
            ))}
            {cursor.isCursorAtEnd() && <SequenceItem onlyCursor forceIndex={sequence.bpCount} renderCursor={showCursor} />}
            {/* <SequenceItem isEnd forceIndex={cursorEndIndex} /> */}
        </div>
    )
}
