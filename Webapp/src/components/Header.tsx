import * as React from 'react'
import styled from '@emotion/styled'
import { transparentize } from 'polished'
import { Link } from 'gatsby'

import { css } from '@emotion/core'
import { heights, dimensions, colors } from '../styles/variables'
import Container from './Container'
import WalletConnect from '../secret/WalletConnect'

const StyledHeader = styled.header`
  height: ${heights.header}px;
  padding: 0 ${dimensions.containerPadding}rem;
  background-color: ${colors.black};
  color: ${transparentize(0.5, colors.white)};
`

const HeaderInner = styled(Container)`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  height: 100%;
  max-width: 100%;
`

const HomepageLink = styled(Link)`
  color: ${colors.white};
  font-size: 1.5rem;
  font-weight: 500;
  &:hover,
  &:focus {
    text-decoration: none;
  }
  margin-left: 2vw;
  margin-right: 2vw;
`

interface HeaderProps {
  title: string
}

const Header: React.FC<HeaderProps> = ({ title }) => {
  return (
    <StyledHeader>
      <HeaderInner>
        <HomepageLink to="/">{title}</HomepageLink>
        <HomepageLink to="/">About</HomepageLink>
        <HomepageLink to="/howto">How to play</HomepageLink>
        <HomepageLink
          css={css`
            color: ${colors.red};
          `}
          to="/game"
        >
          Launch Game
        </HomepageLink>
        {/* <button
          onClick={_ => {
            connect('holodeck-2')
          }}
        >
          {connected ? account?.address : 'Connect'}
        </button> */}
        {/* <button
          onClick={_ => {
            resetViewingKey()
          }}
        >
          Reset
        </button> */}
        <div
          css={css`
            margin-left: auto;
            height: 100%;
            display: flex;
            align-items: center;
          `}
        >
          <WalletConnect chaindId="holodeck-2" />
        </div>
      </HeaderInner>
    </StyledHeader>
  )
}

export default Header
