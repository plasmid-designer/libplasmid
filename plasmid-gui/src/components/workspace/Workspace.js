import styled from 'styled-components'
import PlasmidViewer from '../yapv/PlasmidViewer'

import Editor from './Editor'

const Workspace = ({className}) => {
    return (
        <div className={className}>
            <Editor />
            <PlasmidViewer />
        </div>
    )
}

export default styled(Workspace)`
    display: flex;
    height: 100%;
    overflow: hidden;

    & >:first-child {
        border-right: 2px solid hsl(0,0%,20%);
    }
`
