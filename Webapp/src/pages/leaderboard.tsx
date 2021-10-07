/* eslint-disable @typescript-eslint/camelcase */
import * as React from 'react'
import styled from '@emotion/styled'
import { useContext, useEffect, useState } from 'react'
import IndexLayout from '../layouts'
import Leaderboard from '../components/Leaderboard'
import { KeplrContext } from '../secret/KeplrContext'
import { Contracts, LeaderboardPlayerStats } from '../secret-heroes/contracts'

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
`

export interface LeaderboardsData {
  all_time: LeaderboardPlayerStats[]
  tournament: LeaderboardPlayerStats[]
  tournament_started?: number // "seconds after 01/01/1970 in which the tournament started"
}

const LeaderboardPage = () => {
  const { getQueryClient, connectQuery } = useContext(KeplrContext)

  const [leaderboardsData, setLeaderboardsData] = useState<LeaderboardsData>({
    all_time: [],
    tournament: [],
    tournament_started: undefined
  })

  const updateLeaderboardData = async () => {
    let queryClient = getQueryClient()
    if (queryClient == null) {
      await connectQuery('secret-3')
      queryClient = getQueryClient()
    }
    if (queryClient != null) {
      const newData = await Contracts.arena.leaderboardsQuery(queryClient)
      if (JSON.stringify(newData.leaderboards) !== JSON.stringify(leaderboardsData)) {
        setLeaderboardsData(newData.leaderboards as LeaderboardsData)
      }
    }
  }

  useEffect(() => {
    updateLeaderboardData() // get data once

    const getLeaderboard = setInterval(async () => {
      await updateLeaderboardData()
    }, 30000)

    // clearing interval
    return () => clearInterval(getLeaderboard)
  })

  return (
    <IndexLayout>
      <Container>
        <h2>Leaderboard</h2>
        <h3>All Time Score</h3>
        <Leaderboard leaderboardData={leaderboardsData.all_time} />
      </Container>
    </IndexLayout>
  )
}

export default LeaderboardPage
