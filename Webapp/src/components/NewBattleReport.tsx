import { css } from '@emotion/core'
import React from 'react'
import { Battle } from '../secret-heroes/contracts'
import { colors } from '../styles/variables'

interface BattleReportProps {
  report: Battle
}

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
      `}
    >
      Battle {won ? 'Won' : 'Lost'}
    </div>
  )
}

const NewBattleReportRender: React.FC<BattleReportProps> = ({ report }) => {
  return (
    <div>
      Hello
      <ResultPill won={report.i_won} />
    </div>
  )
}

export default NewBattleReportRender
