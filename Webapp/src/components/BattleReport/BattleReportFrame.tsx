import { css } from '@emotion/core'
import styled from '@emotion/styled'
import { graphql, useStaticQuery } from 'gatsby'
import React, { useState } from 'react'
import ReactTooltip from 'react-tooltip'
import Img from 'gatsby-image'
import { Battle } from '../../secret-heroes/contracts'
import { colors } from '../../styles/variables'

interface BattleReportFrameProps {
  battles: Battle[]
}

const BattleReportColumen = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
  align-items: stretch;
`
const BattleReportEntry = styled.div`
  background-color: ${colors.gray.c900};
  height: 100%;
  display: grid;
  justify-content: start;
  align-content: center;
  border-bottom: 1px solid white;
  font-size: 14px;
  padding: 12px;
`

const BattleReportHeader = styled(BattleReportEntry)`
  font-family: 'Inter', sans-serif;
  font-size: 12px;
  line-height: 20px;
  text-transform: uppercase;
  font-weight: 700;
  padding: 12px;
  background: ${colors.gray.c900};
  border-bottom: 1px solid white;
`

const ResultPill: React.FC<{ won: boolean }> = ({ won }) => {
  const bgColor = won ? colors.cyan : colors.red
  const txtColor = !won ? colors.gray.c200 : colors.gray.c900
  return (
    <div
      css={css`
        border-radius: 15px;
        background: ${bgColor};
        color: ${txtColor}; /* Fallback for older browsers */
        text-align: center;
        width: 90px;
      `}
    >
      Battle {won ? 'Won' : 'Lost'}
    </div>
  )
}

const BattleReportFrame: React.FC<BattleReportFrameProps> = ({ battles }) => {
  const { allImageSharp } = useStaticQuery(graphql`
    query {
      allImageSharp {
        nodes {
          fluid {
            ...GatsbyImageSharpFluid
            originalName
          }
        }
      }
    }
  `)

  const getImage = (id: number) => {
    if (id === 0) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Weapons.png').fluid
    if (id === 1) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Engineering.png').fluid
    if (id === 2) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Biotech.png').fluid
    if (id === 3) return allImageSharp.nodes.find(n => n.fluid.originalName === 'Psychics1.png').fluid
  }
  const itemsPerPage = 5
  const [page, setPage] = useState(0)

  const getName = (nameLong: string) => {
    if (nameLong.length <= 8) return nameLong
    return `${nameLong.slice(0, 8)}...`
  }

  const skillChange = (skillsCurrent: number[], skillsBefore: number[]) => {
    return (
      <div
        css={css`
          display: flex;
          flex-direction: row;
          height: 100%;
          width: 100%;
        `}
      >
        {skillsCurrent.map((v, i) => {
          const changeValue = skillsCurrent[i] - skillsBefore[i]
          // eslint-disable-next-line no-nested-ternary
          const sign = changeValue > 0 ? '+' : ''
          // eslint-disable-next-line no-nested-ternary
          const color = changeValue > 0 ? colors.cyan : changeValue < 0 ? colors.red : 'none'
          const textColor = changeValue <= 0 ? colors.gray.c200 : colors.gray.c900
          return (
            <div
              css={css`
                display: flex;
                flex-direction: column;
                flex: 1 1 10px;
                padding: 10px;
                background: ${color};
                border-radius: 6px;
                margin: 3px;
              `}
            >
              <Img
                css={css`
                  width: 100%;
                  margin: auto;
                  flex-grow: 1;
                  padding: 15px;
                `}
                fluid={getImage(i)}
                alt="Secret Heroes"
              />
              <div
                css={css`
                  font-size: 20;
                  color: ${textColor};
                  text-align: center;
                `}
              >
                {sign}
                {changeValue}
              </div>
            </div>
          )
        })}
      </div>
    )
  }

  const getDate = (utcSeconds: number) => {
    const d = new Date(0)
    d.setUTCSeconds(utcSeconds)
    return (
      <div
        css={css`
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          color: ${colors.gray.c300};
        `}
      >
        {d.toLocaleDateString()} {d.toLocaleTimeString()}
      </div>
    )
  }
  return (
    <div
      css={css`
        display: flex;
        justify-content: flex-start;
      `}
    >
      <div
        css={css`
          display: grid;
          grid-template-columns: 3fr 4fr 1fr 2fr 2.5fr 4fr 6fr;
          border-radius: 6px;
          width: 100%;
          border-collapse: separate;
        `}
      >
        <BattleReportHeader css={css`
          border-top-left-radius: 6px;
        `}>Date</BattleReportHeader>
        <BattleReportHeader>Hero</BattleReportHeader>
        <BattleReportHeader>Type</BattleReportHeader>
        <BattleReportHeader>Your Skill</BattleReportHeader>
        <BattleReportHeader>Winning Skill</BattleReportHeader>
        <BattleReportHeader>Outcome</BattleReportHeader>
        <BattleReportHeader css={css`
          border-top-right-radius: 6px;
        `}>Skill Changes</BattleReportHeader>
        {battles.map(battle => {
          return (
            <>
              <BattleReportEntry>{getDate(battle.timestamp)}</BattleReportEntry>
              <BattleReportEntry data-tip={battle.my_hero.name}>
                <ReactTooltip />
                {getName(battle.my_hero.name)}
              </BattleReportEntry>
              <BattleReportEntry>
                <div>
                  <Img
                    css={css`
                      width: 30px;
                      margin: auto;
                    `}
                    fluid={getImage(battle.skill_used)}
                    alt="Secret Heroes"
                  />
                </div>
              </BattleReportEntry>
              <BattleReportEntry>{battle.my_hero.pre_battle_skills[battle.skill_used]}</BattleReportEntry>
              <BattleReportEntry>{battle.winning_skill_value}</BattleReportEntry>
              <BattleReportEntry>
                <ResultPill won={battle.i_won} />
              </BattleReportEntry>
              <BattleReportEntry>{skillChange(battle.my_hero.post_battle_skills, battle.my_hero.pre_battle_skills)}</BattleReportEntry>
            </>
          )
        })}
      </div>
    </div>
  )
}

export default BattleReportFrame
