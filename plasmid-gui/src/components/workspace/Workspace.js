import styled from 'styled-components'
import ResizableSplitPanel from '../ResizableSplitPanel'
import PlasmidViewer from '../yapv/PlasmidViewer'

import Editor from './Editor'

const Workspace = ({className}) => {
    const resizePanelOptions = {
        left: {
            minWidth: 500,
            startRatio: 0.75,
        }
    }

    return (
        <div className={className}>
            <ResizableSplitPanel
                leftComponent={<Editor />}
                rightComponent={<PlasmidViewer />}
                options={resizePanelOptions}
            />
        </div>
    )
}

export default styled(Workspace)`
    display: flex;
    height: 100%;
    width: 100%;
    overflow: hidden;
`
