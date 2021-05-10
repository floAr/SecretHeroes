
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

