import { css } from '@emotion/core'
import { graphql, Link, navigate, useStaticQuery } from 'gatsby'
import * as React from 'react'

import { library } from '@fortawesome/fontawesome-svg-core'
import { fas } from '@fortawesome/free-solid-svg-icons'
import styled from '@emotion/styled'
import IndexLayout from '../layouts'
import { colors } from '../styles/variables'
import ContentSection from '../components/ContentSection'
import Image from '../components/Image'
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

        <section css={css`
                    display: grid;
                    height: 100vh;
                    align-content: center;
                    justify-content: center;
                    text-align: center;
                    background: #0F1419;
                    //gap: 16px;
                    `}>
          <div class="logo-container" css={css`
            height:250px;
          `}>
            <div className="logo" css={css`
              width:382px;
              height: 249px;
              background: url('images/sh-hero-logo.png');
              background-size: contain;
              background-repeat: no-repeat;
              background-position: center;
              `}>
            </div>
          </div>

          <h2 css={css`
                font-family: Inter;
                font-style: normal;
                font-weight: bold;
                font-size: 24px;
                line-height: 28px;
                text-align: center;
                margin-top: 4px;
                margin-bottom: 0px;
                /* Neutral/#100 */
                color: #F5F9FA;`}>Collect. <span css={css`color:#FC2748;`}>Battle.</span> Dominate.</h2>
          <div class="launch" css={css`
            display: grid;
            gap: 0;
          `}>
            <p css={css`
            margin: 30px 0 0 0;
            font-family: Inter;
            font-style: normal;
            font-weight: normal;
            font-size: 16px;
            //line-height: 28px;
            /* identical to box height, or 175% */

            text-align: center;

            /* Neutral/#200 */

            color: #9AA5B1;`}>Launching In</p>
            <div css={css`
                font-family: Inter;
                font-style: normal;
                font-weight: 800;
                font-size: 60px;
                line-height: 60px;
                /* identical to box height, or 100% */
                text-align: center;
                letter-spacing: -0.025em;
                /* gray/100 */
                color: #F3F4F6;
                text-align: left;
                padding-left: 3.3rem;
              `}>
              <Launch />
            </div>
          </div>
          <StyledButton
            css={css`
                  background: ${colors.red};
                  font-family: Inter;
                  font-style: normal;
                  font-weight: 500;
                  font-size: 16px;
                  line-height: 24px;
                  border: 0;
                  cursor: pointer;
                  min-width: 190px;
                  color: #FFFFFF;
                  padding: 0;
                  display: inline-block;
                  margin: 0 auto;
                  margin-top: 24px;
                  transition: .2s ease;
                  &:hover {
                    background: #a1021a;
                  }
                `}
            onClick={_ => {
              window.open('https://docs.google.com/forms/d/e/1FAIpQLSfrykAtU6PAu1MPfsIYTuphVubVei6nzU1KUOiU6FaEHiJpaw/viewform')
            }}
          >
            Claim Your Hero Now
          </StyledButton>
        </section>

        <ContentSection
          content="In Secret Heroes, each hero comes with a unique skin and a randomly-generated statline across 4 different skills:
          Weapons, Engineering, Biotech, and Psychics. Heroes can be minted in packs for three for just 1 SCRT and there endless
          combinations to collect."
          cta="Mint"
          title="Collect Unique Heroes"
          backgroundImg="images/collect_bg.png"
          mainImg="collect.png"
          visible="grid"
          secondaryImage="images/logo_icon.png"
          secondaryTitle="Based on secretNFTs"
          secondaryContent="Each hero is a secretNFT. This means you own all your heroes and their skin and statlines are private metadata, allowing for privacy-preserving gameplay not found on any other blockchain."
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
      <section css={css`
                    display: grid;
                    align-content: center;
                    justify-content: center;
                    text-align: center;
                    background: #0F1419;
                    gap: 32px;
                    padding-top: 64px;
                    padding-bottom: 64px;

                    `}>

        <h2 css={css`
                font-family: Inter;
                font-style: normal;
                font-weight: 700;
                font-size: 36px;
                line-height: 40px;
                text-align: center;
                margin: 0;
                color: #F5F9FA;
                `}>Want to learn more?</h2>
        <StyledButton
          css={css`
                  background: ${colors.red};
                  font-family: Inter;
                  font-style: normal;
                  font-weight: 500;
                  font-size: 16px;
                  line-height: 24px;
                  border: 0;
                  cursor: pointer;
                  color: #FFFFFF;
                  padding: 0;
                  display: inline-block;
                  margin: 0 auto;
                  margin-top: 20px;
                  transition: .2s ease;
                  cursor: not-allowed;
                  opacity: 0.15;
                `}
          onClick={_ => {
            navigate('https://www.google.com')
          }}
          disabled
        >
          Read Blog Post
          </StyledButton>
      </section>
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
            display: grid;
            grid-auto-flow: column;
            justify-content: space-around;
            align-items: stretch;
            width: 75vw;
            @media only screen and (max-width: 768px) {
              grid-auto-flow: row;
          }
          `}
        >
          <div
            css={css`
              width: 70%;
              display: grid;
              gap: 16px;
              padding-top:64px;
              padding-bottom:64px;
              @media only screen and (max-width: 768px) {
                width: 100%;
            }
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
              margin-top: 0;
              color: #FFFFFF;
              `}
            >
              Secured by Secret Network
            </h3>
            <p
              css={css`
              font-family: Inter;
              font-style: normal;
              font-weight: normal;
              font-size: 18px;
              line-height: 150%;
              color: #FFFFFF;
              `}
            >
              <p>Secret Heroes’ secretNFTs are created using the SNIP-721 standard on Secret Network.</p>

              <p>Secret Network is the first blockchain with privacy-preserving smart contracts. Applications built on Secret Network utilize encrypted data without exposing it to anyone, even the nodes in the network. </p>
            </p>
            <Link to="https://scrt.network/" target="new" css={css`
            font-family: Inter;
            font-style: normal;
            font-weight: bold;
            font-size: 16px;
            line-height: 150%;
            text-decoration-line: underline;
            color: #FFFFFF;
            `}>
              Learn More
              </Link>
          </div>
          <div
            css={css`
              display: grid;
              place-items: center;
              width: 20vw;
              padding: 3vh;
              @media only screen and (max-width: 768px) {
                padding: 0;
                width: 60vw;
                height: 100px;
                order: -1;
                margin-top: 64px;
            }
            `}
          >
            <div
              css={css`
                background: url('images/secret.png');
                width: 100%;
                background-size: 200px;
                background-repeat: no-repeat;
                background-position: left;
                height: 100%;
              `}
            />
          </div>
        </div>
      </div>
      <section clasName="footer" css={css`
      background: #000;
      padding-top: 32px;
      padding-bottom: 32px;
      display: grid;
      text-align: center;
      gap: 16px;
      `}>
        <div className="social" css={css`
          `}>
          <Link to="https://twitter.com/secretheroesnft" target="new" css={css`
          width: 24px;
          height: 24px;
          display: inline-block;
          background: url('images/twitter.png');
          `}></Link>
          <Link to="https://discord.gg/JpTnNRVzpw" target="new" css={css`
          width: 24px;
          height: 24px;
          display: inline-block;
          margin-left: 16px;
          background: url('images/discord.png');
          `}></Link>
        </div>
        <p>© Secret Heroes</p>
      </section>
    </IndexLayout>
  )
}
// <div css={css``}></div>

export default IndexPage
