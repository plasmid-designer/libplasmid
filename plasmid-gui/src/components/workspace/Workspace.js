import styled from 'styled-components'
import { Allotment, setSashSize } from 'allotment'

import ResizableSplitPanel from '../ResizableSplitPanel'
import PlasmidViewer from '../yapv/PlasmidViewer'

import Editor from './Editor'

setSashSize(20)

const Workspace = ({className}) => {
    return (
        <div className={className}>
            <Allotment proportionalLayout vertical={false}>
                <Allotment.Pane minSize={300}>
                    <Editor />
                </Allotment.Pane>
                <Allotment.Pane snap preferredSize={350} maxSize={500}>
                    <PlasmidViewer />
                </Allotment.Pane>
            </Allotment>
        </div>
    )
}

export default styled(Workspace)`
    display: flex;
    height: 100%;
    width: 100%;
    overflow: hidden;

    & > * {
        width: 100%;
    }
`
