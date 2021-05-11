import React from 'react'

import { css } from '@emotion/core';
import styled from '@emotion/styled';
import { colors, dimensions } from '../styles/variables'

const Content = styled.div`
  table {
    background-color: ${colors.gray.c900};
    border: unset;
    border-radius: 6px;
    padding: 0 16px;

    table-layout: fixed;
    width: 100%;

    thead th, tbody td {
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
      th:nth-child(4), th:nth-child(5), th:nth-child(6) {
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
`;

export interface PlayerStats {
  address: string, // player address
  battles: number, // number of battles
  losses: number, // number of losses
  score: number, // players score
  third_in_two_way_ties: number, // numbers of taking 3rd place in a tie
  ties: number, // number ties
  wins: number //number of wins
}

export interface LeaderboardProps {
  all_time: PlayerStats[],
  tournament: PlayerStats[],
  tournament_started: number // "seconds after 01/01/1970 in which the tournament started"
}

const leaderboard = [
  {
    address: 'secret1jfdkfj2399fjjfd9jfijfeifjfi',
    score: 200,
    battles: 10,
    wins: 2
  },
  {
    address: 'secretdfffjifjejfijefiefjjfejifejf',
    score: 100,
    battles: 20,
    wins: 12
  }
]

const Leaderboard: React.FC<LeaderboardProps> = ({ all_time }) => {
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
          {leaderboard.map((it, index) =>
            <tr>
              <td>#{ index + 1 }</td>
              <td>{ it.address }</td>
              <td>{ it.score }</td>
              <td>{ it.battles }</td>
              <td>{ it.wins }</td>
              <td>{ it.wins / it.battles * 100 }%</td>
            </tr>
          )}
        </tbody>
      </table>
    </Content>
  )
}

export default Leaderboard
