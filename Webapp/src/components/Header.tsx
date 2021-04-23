import * as React from 'react'
import styled from '@emotion/styled'
import { transparentize } from 'polished'
import { Link } from 'gatsby'

import { heights, dimensions, colors } from '../styles/variables'
import Container from './Container'
import { KeplrContext } from '../secret/KeplrContext'
import { css } from '@emotion/core'

const StyledHeader = styled.header`
  height: ${heights.header}px;
  padding: 0 ${dimensions.containerPadding}rem;
  background-color: ${colors.black};
  color: ${transparentize(0.5, colors.white)};
`

const HeaderInner = styled(Container)`
  display: flex;
  flex-direction: row;
  align-items: baseline;
  justify-content: space-evenly;
  height: 100%;
`

const HomepageLink = styled(Link)`
  color: ${colors.white};
  font-size: 1.5rem;
  font-weight: 600;

  &:hover,
  &:focus {
    text-decoration: none;
  }
`

interface HeaderProps {
  title: string
}

const Header: React.FC<HeaderProps> = ({ title }) => {
  const { connect, account, connected, resetViewingKey } = React.useContext(KeplrContext)
  return (
    <StyledHeader>
      <HeaderInner>
        <HomepageLink to="/">{title}</HomepageLink>
        <p>About</p>
        <p>How to play</p>
        <p
          css={css`
            justify-self: flex-end;
            color: ${colors.red};
          `}
        >
          Launch Game
        </p>
        <button
          onClick={_ => {
            connect('holodeck-2')
          }}
        >
          {connected ? account?.address : 'Connect'}
        </button>
        <button
          onClick={_ => {
            resetViewingKey()
          }}
        >
          Reset
        </button>
      </HeaderInner>
    </StyledHeader>
  )
}

export default Header
