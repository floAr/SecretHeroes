import { css } from '@emotion/core'
import { graphql, useStaticQuery } from 'gatsby'
import React from 'react'

import Img from 'gatsby-image'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { Battle } from '../secret-heroes/contracts'

interface BattleReportProps {
  report: Battle
}

const BattleReportRender: React.FC<BattleReportProps> = ({ report }) => {
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

  // const getUpgradeValue = (stats: HeroStats, index: number) => {
  //   return stats.current[index] - stats.base[index]
  // }

  const getImage = id => {
    if (id === 0) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Weapons.png').fluid
    if (id === 1) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Engineering.png').fluid
    if (id === 2) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Biotech.png').fluid
    if (id === 3) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Psychics1.png').fluid
  }

  const getName = (nameLong: string) => {
    if (nameLong.length <= 5) return nameLong
    return `${nameLong.slice(0, 5)}...`
  }

  return (
    <div
      css={css`
        display: flex;
        flex-direction: row;
        align-content: space-around;
        margin: 5px;
        background-color: ${report.i_won ? '#73fc58' : '#ff6464'};
        align-items: center;
      `}
    >
      <Img
        css={css`
          width: 50px;
          padding: 3px;
          margin: 5px;
        `}
        fluid={getImage(report.skill_used)}
        alt="Secret Heroes"
      />
      <div
        css={css`
          margin: 3px;
          border-right: 1px solid black;
        `}
      >
        Hero {getName(report.my_hero.name)}
      </div>

      <div
        css={css`
          display: flex;
          flex-direction: column;
          padding: 3px;
          align-items: center;
          border-right: 1px solid black;
        `}
      >
        <div>You:</div>
        <div>{report.my_hero.pre_battle_skills[report.skill_used]}</div>
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: column;
          margin: 3px;
          align-items: center;
        `}
      >
        <div>Winner:</div>
        <div>{report.winning_skill_value}</div>
      </div>
    </div>
  )
}

export default BattleReportRender
