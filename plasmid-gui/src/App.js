import styled from 'styled-components'
import { RecoilRoot } from 'recoil'

import Header from './components/header/Header'
import Workspace from './components/workspace/Workspace'

const App = ({ className }) => {
  return (
    <RecoilRoot>
        <div className={className}>
            <Header />
            <Workspace />
        </div>
    </RecoilRoot>
  );
}

export default styled(App)`
    display: flex;
    flex-direction: column;
    height: 100vh;
`
