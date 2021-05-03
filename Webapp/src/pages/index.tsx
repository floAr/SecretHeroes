import { css } from '@emotion/core'
import { graphql, Link, navigate, useStaticQuery } from 'gatsby'
import * as React from 'react'

import { library } from '@fortawesome/fontawesome-svg-core'
import { fas } from '@fortawesome/free-solid-svg-icons'
import styled from '@emotion/styled'
import IndexLayout from '../layouts'
import { colors } from '../styles/variables'
import ContentSection from '../components/ContentSection'
import Launch from '../components/Launch'

library.add(fas)

const SlatedSection = styled.section`
  width: 45vw;
  position: relative;
  padding: 150px 0;
  background: ${colors.black};
  overflow: visible;
  z-index: 1;
  display: flex;
  flex-direction: column;

  /* or 107% */
  letter-spacing: -0.025em;

  &:after {
    content: '';
    width: 120%;
    height: 100%;
    position: absolute;
    background: inherit;
    z-index: -1;
    bottom: 0;
    transform-origin: left bottom;
    transform: skewX(-7deg);
  }
`

const KeywordDiv = styled.div`
  margin-top: 5px;
  /* font-family: Zen Dots; */
  font-style: bold;
  font-weight: normal;
  font-size: 50px;
  line-height: 80%;
  font-weight: 800;

  /* or 63px */
  letter-spacing: -0.025em;
  text-transform: uppercase;

  color: #f5f9fa;
`

const StyledButton = styled.button`
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  padding: 17px 41px;

  min-width: 160px;
  width: 10vw;
  height: 62px;
  left: 3px;
  top: 3px;

  font-family: Inter;
  font-style: normal;
  font-weight: bold;
  font-size: 20px;
  line-height: 26px;
  margin-right: 1vw;

  background: ${colors.gray.c200};
  /* box-shadow: 0px 1px 3px rgba(50, 63, 75, 0.1), 0px 1px 2px rgba(50, 63, 75, 0.06); */
  border-radius: 6px;
`

const Spacer = styled.div`
  width: 10vw;
  background: ${colors.black};
`

const IndexPage = () => {
  const data = useStaticQuery(graphql`
    query {
      file(relativePath: { eq: "secret-heroes.png" }) {
        childImageSharp {
          # Specify a fixed image and fragment.
          # The default width is 400 pixels
          fixed(width: 150) {
            ...GatsbyImageSharpFixed
          }
          fluid {
            ...GatsbyImageSharpFluid
          }
        }
      }
    }
  `)
  const isBrowser = typeof window !== 'undefined'

  return (
    <IndexLayout>
      <div
        css={css`
          display: flex;
          flex-direction: column;
          width: 100%;
        `}
      >
        <div
          css={css`
            display: flex;
            flex-direction: row;
            width: 100%;
          `}
        >
          <Spacer />
          <SlatedSection>
            <KeywordDiv>Collect Heroes.</KeywordDiv>
            <KeywordDiv
              css={css`
                color: ${colors.red};
              `}
            >
              Battle Other Players.
            </KeywordDiv>
            <KeywordDiv>Dominate the Arena.</KeywordDiv>
            <div
              css={css`
                font-style: normal;
                font-weight: 400;
                font-size: 20px;
                line-height: 28px;

                /* or 140% */

                /* Neutral/#100 */
                color: #f5f9fa;

                /* Inside Auto Layout */
                flex: none;
                align-self: stretch;
                flex-grow: 0;
                margin: 20px 0px;
                max-width: 33vw;
              `}
            >
              Welcome to Secret Heroes, an auto-battler based on collectible secretNFTs.
            </div>
            <div
              css={css`
                display: flex;
                justify-content: end;
                flex-wrap: wrap;
                margin: -10px 0 0 0;

                > * {
                  margin-top: 10px;
                }
              `}
            >
              <StyledButton
                css={css`
                  background: ${colors.red};
                  color: ${colors.gray.c200};
                `}
                onClick={_ => {
                  navigate('/connect')
                }}
                disabled
              >
                <Launch />
              </StyledButton>

              <StyledButton
                onClick={_ => {
                  navigate('/howto')
                }}
              >
                How to play
              </StyledButton>
            </div>
          </SlatedSection>
          <div
            css={css`
              background: url('images/banner.png');
              background-size: cover;
              background-position: 50% 30%;
              width: 100%;
            `}
          />
        </div>

        <ContentSection
          content="In Secret Heroes, each hero comes with a unique skin and a randomly-generated statline across 4 different skills:
          Weapons, Engineering, Biotech, and Psychics. Heroes can be minted in packs for three for just 1 SCRT and there endless
          combinations to collect."
          cta="Mint"
          title="Collect Unique Heroes"
          backgroundImg="images/collect_bg.png"
          mainImg="collect.png"
        />

        <ContentSection
          content="Choose a hero and send it to battle against to two other players. In each battle, the game randomly chooses a skill.
          The hero with the highest stat for the chosen skills is crowned the victor."
          cta="Battle"
          title="Fight Other Players"
          backgroundImg="images/fight_bg.png"
          mainImg="fight.png"
        />

        <ContentSection
          content="Each time you win a battle, your hero’s stats will improve. The tougher the opponent  you beat, the greater the stat
          increase. Be careful though, losing battles can cause you to lose stats*"
          cta="Win"
          title="Build the Best Hero"
          backgroundImg="images/build_bg.png"
          mainImg="build.png"
        />
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: column;
          align-items: center;
          background: black;
        `}
      >
        <div
          css={css`
            display: flex;
            flex-direction: row;
            justify-content: space-around;
            align-items: stretch;
            width: 75vw;
          `}
        >
          <div
            css={css`
              width: 50%;
              display: flex;
              flex-direction: column;
            `}
          >
            <h3
              css={css`
                font-family: Inter;
                font-style: normal;
                font-weight: bold;
                font-size: 24px;
                line-height: 29px;

                /* identical to box height */
                letter-spacing: -0.5px;

                color: #ffffff;
              `}
            >
              Secured by Secret Network
            </h3>
            <p
              css={css`
                font-family: Roboto Mono;
                font-style: normal;
                font-weight: normal;
                font-size: 16px;
                line-height: 150%;
              `}
            >
              Secret Heroes’ secretNFTs are created using the SNIP-721 standard on Secret Network. Secret Network is the first blockchain
              with privacy-preserving smart contracts. Applications built on Secret Network utilize encrypted data without exposing it to
              anyone, even the nodes in the network.
            </p>
            <Link to="https://scrt.network/" target="new">
              Learn More
            </Link>
          </div>
          <div
            css={css`
              display: grid;
              place-items: center;
              width: 20vw;
              padding: 3vh;
            `}
          >
            <div
              css={css`
                background: url('images/secret.png');
                width: 100%;
                background-size: contain;
                background-repeat: no-repeat;
                background-position: 50% 50%;
                height: 100%;
              `}
            />
          </div>
        </div>
      </div>
    </IndexLayout>
  )
}
// <div css={css``}></div>

export default IndexPage
