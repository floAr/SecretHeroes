import { css } from '@emotion/core'
import { graphql, useStaticQuery } from 'gatsby'
import React from 'react'
import Img from 'gatsby-image'
import { BattleState } from '../pages/game'

interface BattleReportProps {
  report: BattleState
}

const BattleStateRender: React.FC<BattleReportProps> = ({ report }) => {
  const { allImageSharp } = useStaticQuery(graphql`
    query {
      allImageSharp {
        nodes {
          fixed(width: 150) {
            ...GatsbyImageSharpFixed
            originalName
          }
          fluid {
            ...GatsbyImageSharpFluid
            originalName
          }
        }
      }
    }
  `)

  const getImage = id => {
    if (id === 0) return allImageSharp.nodes.find((n: { fluid: { originalName: string } }) => n.fluid.originalName === 'Weapons.png').fluid
    if (id === 1) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Engineering.png').fluid
    if (id === 2) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Biotech.png').fluid
    if (id === 3) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Psychics1.png').fluid
  }

  return (
    <div
      css={css`
        display: flex;
        flex-direction: column;
        align-content: space-around;
      `}
    >
      <h5>Hero {report.your_hero?.name} </h5>
      <div
        css={css`
          display: flex;
          flex-direction: row;
          align-content: space-around;
          justify-content: space-between;
          align-items: baseline;
        `}
      >
        <Img
          key={`${report.your_hero?.name}wea`}
          css={css`
            width: 50px;
            height: 50px;
          `}
          fluid={getImage(0)}
          alt="Secret Heroes"
        />
        <h5>{report.your_hero?.weapons}</h5>
        <Img
          key={`${report.your_hero?.name}eng`}
          css={css`
            width: 50px;
            height: 50px;
          `}
          fluid={getImage(1)}
          alt="Secret Heroes"
        />
        <h5>{report.your_hero?.engineering}</h5>
        <Img
          key={`${report.your_hero?.name}bio`}
          css={css`
            width: 50px;
            height: 50px;
          `}
          fluid={getImage(2)}
          alt="Secret Heroes"
        />
        <h5>{report.your_hero?.biotech}</h5>
        <Img
          key={`${report.your_hero?.name}psy`}
          css={css`
            width: 50px;
            height: 50px;
          `}
          fluid={getImage(3)}
          alt="Secret Heroes"
        />
        <h5>{report.your_hero?.psychics}</h5>
      </div>
    </div>
  )
}

export default BattleStateRender
