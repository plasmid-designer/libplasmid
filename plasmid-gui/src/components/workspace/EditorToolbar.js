import styled from 'styled-components'
import { useRecoilState } from 'recoil'

import { editorHintState } from '../../state/atoms'

const EditorToolbar = ({ className }) => {
    const [editorHints, setEditorHints] = useRecoilState(editorHintState)

    return (
        <div className={className}>
            <div className="input_container">
                <div className="title">View</div>
                <div className="input_wrapper">
                    <input
                        type="checkbox"
                        id="show-complement"
                        checked={editorHints.showComplementStrand}
                        onChange={e => setEditorHints(hints => ({...hints, showComplementStrand: e.target.checked}))}
                    />
                    <label htmlFor="show-complement">Antistrand</label>
                </div>
                <div className="input_wrapper">
                    <input
                        type="checkbox"
                        id="show-codon-numbers"
                        checked={editorHints.showCodonNumbers}
                        onChange={e => setEditorHints(hints => ({...hints, showCodonNumbers: e.target.checked}))}
                    />
                    <label htmlFor="show-codon-numbers">Codon Indices</label>
                </div>
                <div className="input_wrapper">
                    <input
                        type="checkbox"
                        id="show-peptides"
                        checked={editorHints.showPeptides}
                        onChange={e => setEditorHints(hints => ({...hints, showPeptides: e.target.checked}))}
                    />
                    <label htmlFor="show-peptides">Peptides</label>
                </div>
                <div className="input_wrapper">
                    <input
                        type="checkbox"
                        id="highlight-active-codon"
                        checked={editorHints.highlightCurrentCodon}
                        onChange={e => setEditorHints(hints => ({...hints, highlightCurrentCodon: e.target.checked}))}
                    />
                    <label htmlFor="highlight-active-codon">Highlight Active Codon</label>
                </div>
            </div>
            <div className="input_container" style={{flexGrow: 1}}></div>
        </div>
    )
}

export default styled(EditorToolbar)`
    display: flex;
    height: 2.5rem;
    align-items: center;
    background: hsl(0,0%,98%);
    border-bottom: 1px solid hsl(0,0%,50%);
    font-size: 10pt;

    & .input_container {
        display: flex;
        align-items: center;
        gap: .25rem;
        height: 100%;
        border-radius: .25rem;

        & > .title {
            display: flex;
            align-items: center;
            padding: 0 .25rem 0 .75rem;
            font-weight: 500;
            border-left: 1px solid hsl(0,0%,90%);
            height: 100%;
        }

        .input_wrapper {
            display: flex;
            align-items: center;
            border-radius: .25rem;
            /* background: hsl(0,0%,95%); */
            padding: 0 .25rem;
            /* border: 1px solid hsl(0,0%,75%); */
            height: 100%;

            label {
                user-select: none;
            }

            &:last-child {
                margin-right: .5rem;
            }
        }
    }
`
