import { css } from '@emotion/core'
import { graphql, useStaticQuery } from 'gatsby'
import React from 'react'
import { BattleResult } from '../pages'
import Img from "gatsby-image"

interface BattleReportProps {
  report: BattleResult
}

const BattleReportRender: React.FC<BattleReportProps> = ({ report }) => {
  const { allImageSharp } = useStaticQuery(graphql`
    query {
      allImageSharp {
        nodes {
          fixed (width: 150){
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
    if (id === 0) return allImageSharp.nodes.find(n => n.fluid.originalName === "Weapons.png")
      .fluid
    if (id === 1) return allImageSharp.nodes.find(n => n.fluid.originalName === "Engineering.png").fluid
    if (id === 2) return allImageSharp.nodes.find(n => n.fluid.originalName === "Biotech.png").fluid
    if (id === 3) return allImageSharp.nodes.find(n => n.fluid.originalName === "Psychics1.png").fluid
  }

  return (
    <div css={css`display:flex; flex-direction:row; align-content:space-around;margin:5px;background-color: ${report.i_won ? '#73fc58' : '#ff6464'};`}>
      <Img
        css={css`
          width: 50px;
          height: 50px;
        `}
        fluid={getImage(report.skill_used)}
        alt="Secret Heroes"
      />
      <h5>Hero {report.my_hero} | Your Skill: {report.my_skills[report.skill_used]} {!report.i_won ? " Other: " + report.winning_skill_value : ""} | {report.i_won ? "WIN" : "LOOSE"}</h5>
    </div>
  )
}

export default BattleReportRender
