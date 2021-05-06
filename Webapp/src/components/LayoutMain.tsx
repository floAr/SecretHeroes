import * as React from 'react'
import styled from '@emotion/styled'
import Header from './Header'
import { colors } from '../styles/variables'

const StyledLayoutMain = styled.main`
  display: flex;
  flex-direction: column;
  flex: 1;
`

interface LayoutMainProps {
  className?: string
}

const LayoutMain: React.FC<LayoutMainProps> = ({ children, className }) => (
  <StyledLayoutMain className={className}>
    {/* <Header title={'Secret Heroes'} /> */}
    {children}
  </StyledLayoutMain>
)

export default LayoutMain
