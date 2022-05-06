import { memo, useMemo } from 'react'
import { useRecoilValue } from 'recoil'
import styled from 'styled-components'

import { editorHintState } from '../../../state/atoms'
import ColorUtil from '../../../util/ColorUtil'

const _Cursor = memo(({ className, offset }) => {
    return (
        <div className={className} />
    )
})

const Cursor = styled(_Cursor)`
    position: absolute;
    top: calc(50% - 0.5rem);
    display: flex;
    width: 1px;
    height: 1rem;
    background-color: black;
    user-select: none;
    pointer-events: none;
    animation: cursor 1s infinite;
    left: calc(${props => props.offset} * 0.65rem);

    @keyframes cursor {
        0% { opacity: 1; }
        49% { opacity: 1; }
        50% { opacity: 0; }
        99% { opacity: 0; }
        100% { opacity: 1; }
    }
`

/**
 * @param {{
 * className: string,
 * letter: string,
 * color: string,
 * index: number,
 * inSelection: boolean,
 * }} props
 */
const _Nucleotide = ({ className, letter, index }) => {
    return (
        <div className={className} data-index={index}>
            {letter}
        </div>
    )
}

const Nucleotide = memo(
    styled(_Nucleotide)`
        position: relative;
        color: ${props => props.color};
        background: ${props => props.inSelection ? 'hsla(327, 20%, 67%, 0.5)' : 'transparent'};
    `
)

/**
 * @param {{
 * className: string,
 * letter: string,
 * color: string,
 * index: number,
 * }} props
 */
const _Peptide = ({ className, letter, index, color }) => {
    return (
        <div className={className} data-index={index}>
            <span>{letter}</span>
        </div>
    )
}

const Peptide = memo(
    styled(_Peptide)`
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 10pt;
        padding: .1rem;
        z-index: 1;

        & > span {
            display: flex;
            justify-content: center;
            align-items: center;
            width: 100%;
            height: 100%;
            border-radius: .25rem;
            background: ${props => props.color};
        }
    `
)

/**
 * @param {{
 * className: string,
 * number: string,
 * index: number,
 * }} props
 */
const _CodonIndex = ({ className, number, index }) => {
    return (
        <div className={className} data-index={index}>
            {number}
        </div>
    )
}

const CodonIndex = memo(
    styled(_CodonIndex)`
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 10pt;
    `
)

/**
 * @param {{
 * className: string,
 * index: number,
 * cursor: import('../SequenceDataModel').SequenceDataCursorModel,
 * selection: import('../SequenceDataModel').SequenceDataSelectionModel,
 * letters: string[],
 * colors: string[],
 * opacity: number,
 * }} props
 */
const _Codon = props => {
    const { className, index, cursor, selection, letters, colors } = props
    // console.log(`[${index}] ${letters}`, structuredClone(props))
    return (
        <div className={className} data-index={index}>
            {letters.map((letter, nucIndex) => (
                <>
                    <Nucleotide
                        key={nucIndex}
                        index={index + nucIndex}
                        letter={letter}
                        color={colors[nucIndex]}
                        inSelection={selection.contains(index + nucIndex)}
                    />
                    {cursor.cursorPosition === index + nucIndex && (
                        <Cursor offset={nucIndex} />
                    )}
                </>
            ))}
        </div>
    )
}

const Codon = memo(
    styled(_Codon)`
        position: relative;
        display: flex;
        flex-flow: row;
        opacity: ${props => props.opacity};
    `,
    (lastProps, nextProps) => {
        // Short circuit if the index or content changed
        if (
            lastProps.index !== nextProps.index ||
            lastProps.letters !== nextProps.letters
        ) return false

        // Diff cursor rendering
        const shouldRenderCursor = ({cursor, index, letters}) => (
            cursor.cursorPosition >= index &&
            cursor.cursorPosition < index + letters.length
        )
        const nextShouldRenderCursor = shouldRenderCursor(nextProps)
        const renderCursorChanged = shouldRenderCursor(lastProps) !== nextShouldRenderCursor
        if (renderCursorChanged) return false

        // Diff cursor position
        const cursorPositionChanged = (
            !renderCursorChanged &&
            nextShouldRenderCursor &&
            lastProps.cursor.cursorPosition !== nextProps.cursor.cursorPosition
        )
        if (cursorPositionChanged) return false

        // Diff selection overlap count
        const overlapCount = ({selection, index, letters}) => selection.overlapCount(index, letters.length)
        if (overlapCount(lastProps) !== overlapCount(nextProps)) return false

        return true
    }
)

/**
 * @param {{
 * className: string,
 * item: import('../SequenceDataModel').SequenceDataItemModel,
 * cursor: import('../SequenceDataModel').SequenceDataCursorModel,
 * selection: import('../SequenceDataModel').SequenceDataSelectionModel,
 * }} props
 */
const _SequenceItem = ({ className, item, cursor, selection }) => {
    const editorHints = useRecoilValue(editorHintState)

    const codonColors = useMemo(
        () => ColorUtil.getCodonColors(item.codonLetters),
        [item.codonLetters]
    )
    const anticodonColors = useMemo(
        () => ColorUtil.getCodonColors(item.anticodonLetters),
        [item.anticodonLetters]
    )
    const peptideColor = useMemo(
        () => ColorUtil.getPeptideColor(item.peptideLetter),
        [item.peptideLetter]
    )
    const highlightCodon = useMemo(
        () => editorHints.highlightCurrentCodon && cursor.isItemSelected(item),
        [editorHints.highlightCurrentCodon, cursor, item]
    )

    return (
        <div className={className} data-index={item.startIndex} data-highlighted={highlightCodon}>
            <Codon
                index={item.startIndex}
                selection={selection}
                colors={codonColors}
                letters={item.codonLetters}
                cursor={cursor}
                opacity={1}
            />
            {editorHints.showComplementStrand && (
                <Codon
                    index={item.startIndex}
                    selection={selection}
                    colors={anticodonColors}
                    letters={item.anticodonLetters}
                    cursor={cursor}
                    opacity={0.5}
                />
            )}
            {editorHints.showCodonNumbers && (
                <CodonIndex
                    index={item.startIndex}
                    number={Math.floor(item.startIndex / 3)}
                />
            )}
            {editorHints.showPeptides && (
                <Peptide
                    index={item.startIndex}
                    letter={item.peptideLetter}
                    color={peptideColor}
                />
            )}
        </div>
    )
}

const SequenceItem = memo(
    styled(_SequenceItem)`
        display: flex;
        flex-flow: column;
        border-radius: .25rem;
        position: relative;

        &[data-highlighted=true]:before {
            content: " ";
            position: absolute;
            top: 0;
            right: 1px;
            bottom: 2px;
            left: 1px;
            border: 1px solid hsla(0,0%,50%,.33);
            border-radius: .25rem;
            pointer-events: none;
            user-select: none;
            z-index: 0;
        }
    `
)

/**
 * @param {{
 * className: string,
 * sequence: import('../SequenceDataModel').default,
 * cursor: import('../SequenceDataModel').SequenceDataCursorModel,
 * selection: import('../SequenceDataModel').SequenceDataSelectionModel,
 * showCursor: boolean,
 * }} props
 */
const NextRenderer = ({
    className,
    sequence,
    cursor,
    selection,
    showCursor,
}) => {
    return (
        <div className={className}>
            {sequence.items.map(item => (
                <SequenceItem
                    key={item.startIndex}
                    item={item}
                    cursor={cursor}
                    selection={selection}
                />
            ))}
        </div>
    )
}

export default styled(NextRenderer)`
    display: flex;
    flex-flow: row wrap;
    font-family: monospace;
    font-size: 14pt;
    cursor: text;
    width: 100%;
`
