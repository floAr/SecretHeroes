import * as React from 'react'

import IndexLayout from '../layouts'

import Leaderboard from '../components/Leaderboard.tsx';

import { css } from '@emotion/core';
import styled from '@emotion/styled';
import { colors } from '../styles/variables'

const Container = styled.div`
  display: block;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1.5rem;
  margin-top: 100px;

  h2 {
    // Fonts
    font-size: 60px;
    font-weight: bold;
    line-height: 60px;
    color: white;

    margin-bottom: 72px;
  }

  h3 {
    // Fonts
    font-size: 24px;
    font-weight: bold;
    line-height: 30px;
    color: white;

    margin-bottom: 24px;
  }
`;

const LeaderboardPage = () => (
  <IndexLayout>
    <Container>
      <h2>Leaderboard</h2>
      <h3>All Time Score</h3>

      <Leaderboard/>
    </Container>
  </IndexLayout>
)

export default LeaderboardPage
