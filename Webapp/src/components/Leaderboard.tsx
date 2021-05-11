import React, { useContext } from 'react'
import { css } from '@emotion/core'
import styled from '@emotion/styled'
import { colors, dimensions } from '../styles/variables'
import { LeaderboardPlayerStats } from '../secret-heroes/contracts'
import { KeplrContext } from '../secret/KeplrContext'

const Content = styled.div`
  table {
    background-color: ${colors.gray.c900};
    border: unset;
    border-radius: 6px;
    padding: 0 16px;

    table-layout: fixed;
    width: 100%;

    thead th,
    tbody td {
      padding: 12px 24px;
    }

    thead {
      th {
        border: unset;
        border-bottom: 1px solid ${colors.gray.c600};
        text-transform: uppercase;

        font-size: ${dimensions.fontSize.extraSmall}px;
      }

      // Fixed columns size (a little bit weird)
      th:nth-child(1) {
        width: 10%;
      }
      th:nth-child(2) {
        width: 30%;
      }
      th:nth-child(3) {
        width: 15%;
      }
      th:nth-child(4),
      th:nth-child(5),
      th:nth-child(6) {
        width: 15%;
      }
    }

    tbody {
      td {
        background-color: ${colors.gray.c900};
        border: unset;
        border-bottom: 1px solid ${colors.gray.c600};

        overflow: hidden;
        text-overflow: ellipsis;

        font-size: ${dimensions.fontSize.small}px;
      }
    }
  }
`
export interface LeaderboardProps {
  leaderboardData: LeaderboardPlayerStats[]
}

const Leaderboard: React.FC<LeaderboardProps> = ({ leaderboardData }) => {
  const { account } = useContext(KeplrContext)
  return (
    <Content>
      <table>
        <thead>
          <tr>
            <th>Rank</th>
            <th>Address</th>
            <th>Score</th>
            <th>Battles</th>
            <th>Wins</th>
            <th>Win %</th>
          </tr>
        </thead>

        <tbody>
          {leaderboardData.map((it, index) => (
            <tr
              css={
                account?.address != null && account.address === it.address
                  ? css`
                      font-weight: bold;
                    `
                  : css``
              }
            >
              <td>#{index + 1}</td>
              <td>{it.address}</td>
              <td>{Math.max(0, it.score)}</td>
              <td>{it.battles}</td>
              <td>{it.wins}</td>
              <td>{(it.wins / it.battles) * 100}%</td>
            </tr>
          ))}
        </tbody>
      </table>
    </Content>
  )
}

export default Leaderboard
