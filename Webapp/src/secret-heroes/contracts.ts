/* eslint-disable @typescript-eslint/camelcase */

import { toast } from "react-toastify"
// eslint-disable-next-line prettier/prettier
import type { CosmWasmClient, SigningCosmWasmClient } from "secretjs"
// eslint-disable-next-line import/no-unresolved
import { Coin, StdFee } from "secretjs/types/types"


export type HumanAddr = string
export type StatusResponse = 'Success' | 'Failure'

export interface MintMsg {
  mint: {
    names: string[]
  }
}

export interface MintRsp {
  mint: {
    status: StatusResponse
  }
}


export interface FightStatusQry {
  bullpen: {
    address: HumanAddr,
    viewing_key: string
  }
}
export type HeroStats = {
  base: number[]
  current: number[]
}
export type TokenInfo = {
  address: HumanAddr,
  token_id: string

}
export type WaitingHero = {
  name: string,
  stats: HeroStats
  token_info: TokenInfo
}

export interface FightStatusRsp {
  bullpen: {
    heroes_waiting: number,
    your_hero?: WaitingHero
  }
}

export interface BattleHistoryQry {
  battle_history: {
    address: HumanAddr,
    viewing_key: string,
    page?: number,
    page_size?: number
  }
}

export type Hero = {
  name: string
  post_battle_skills: number[]
  pre_battle_skills: number[]
  token_info: TokenInfo
}

export type Battle = {
  battle_number: number
  i_won: boolean
  my_hero: Hero
  skill_used: number
  winning_skill_value: number,
  timestamp: number
}

export interface BattleHistoryRsp {
  battle_history: {
    history: Battle[]
  }
}

export interface CreateViewingkeyMsg {
  create_viewing_key: {
    entropy: string
  }
}

export interface SetViewingkeyMsg {
  set_viewing_key: {
    key: string
    padding?: string
  }
}

export interface ViewingKeyRsp {
  viewing_key: {
    key: string
  }
}

export interface WithdrawMsg {
  chicken_out: {

  }
}

export interface WithdrawRsp {
  chicken_out: {
    message: string
  }
}

export interface SendHeroMsg {
  send_nft: {
    contract: HumanAddr,
    token_id: string,
    msg: string
  }
}

export interface SendHeroRsp {
  send_nft: {
    status: StatusResponse
  }
}

export interface GetAllTokensQry {
  tokens: {
    owner: HumanAddr,
    viewing_key: string
  }
}

export interface GetAllTokensRsp {
  token_list: {
    tokens: string[]
  }
}

export type ViewerInfo = {
  address: HumanAddr,
  viewing_key: string
}
export interface GetTokenInfoQry {
  private_metadata: {
    token_id: string,
    viewer: ViewerInfo
  }
}

export interface GetTokenInfoRsp {
  private_metadata: {
    description: string,
    image: string,
    name: string
  }
}

export interface GetLeaderboardsQry {
  leaderboards: {}
}

export type LeaderboardPlayerStats = {
  address: string // player address
  battles: number // number of battles
  losses: number // number of losses
  score: number // players score
  third_in_two_way_ties: number // numbers of taking 3rd place in a tie
  ties: number // number ties
  wins: number // number of wins
}

export interface GetLeaderboardsRsp {
  leaderboards: {
    all_time: LeaderboardPlayerStats[]
    tournament: LeaderboardPlayerStats[]
    tournament_started: number
  }
}

const randomString = (length: number) => {
  let result = ''
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  const charactersLength = characters.length
  for (let i = 0; i < length; i += 1) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength))
  }
  return result
}

export const getEntropy = () => {
  return btoa(randomString(15))
}

const createPadding = (currentLength: number, targetLength: number) => {
  return randomString(targetLength - currentLength)
}

const getFees = (txName: 'mint' | 'viewingKeys' | 'send' | 'pull'): StdFee => {

  let gas = "1000000";
  let amount = "0";
  const denom = "uscrt";
  switch (txName) {
    case "mint":
      gas = "650000";
      break;
    case "viewingKeys":
      gas = "120000";
      break;
    case "send":
      gas = "860000";
      break;
    case "pull":
      gas = "260000";
      break;
    default:
      break

  }
  if (amount === "0") {
    amount = gas;
  }
  return {
    amount: [{ amount, denom }],
    gas
  }
  // return {
  //   exec: {
  //     amount: [{ amount, denom }],
  //     gas
  //   }
  // }
}

async function query<IN extends object, OUT>(client: CosmWasmClient, address: HumanAddr, query_msg: IN): Promise<OUT | undefined> {
  try {
    const resp = await client.queryContractSmart(address, query_msg)
    return resp
  }
  catch (err) {
    //  toast.error(`Error running query: ${err}`)
    return undefined
  }
}

async function transact<IN extends object, OUT>
  (client: SigningCosmWasmClient, address: HumanAddr, msg: IN, transfer?: Coin, fee?: StdFee): Promise<OUT | undefined> {
  try {
    let resp
    if (transfer !== undefined)
      resp = (await client.execute(address, msg, '', [transfer], fee))
    else
      resp = (await client.execute(address, msg, '', [], fee))
    return JSON.parse(new TextDecoder().decode(resp.data)) as OUT
  }
  catch (err) {
    //  toast.error(`Error running query: ${err}`)
    return undefined
  }
}

const minter_address = "secret19lk0jeh7msdqmnaql5aatruhz4hajacut0l4pg" as HumanAddr
const arena_address = "secret123cvl7awpvs45lztcn6v343arssse35vmywnnj" as HumanAddr
const card_address = "secret1ytu2a642jwysle87eu89vqxqlfveajsu00r8v4" as HumanAddr

export const mintHeroes = async (client: SigningCosmWasmClient, names: string[]): Promise<MintRsp | undefined> => {
  return transact<MintMsg, MintRsp>(client, minter_address, {
    mint: {
      names
    }
  }, {
    denom: 'uscrt', amount: "1000000"
  },
    getFees('mint'))
}

export const fightStatus = async (client: CosmWasmClient, address: HumanAddr, viewingKey: string): Promise<FightStatusRsp | undefined> => {
  return query<FightStatusQry, FightStatusRsp>(client, arena_address, {
    bullpen: {
      address,
      viewing_key: viewingKey
    }
  })
}
export const fightHistory =
  async (client: CosmWasmClient, address: HumanAddr, viewingKey: string, paging?: { page: number, page_size: number }):
    Promise<BattleHistoryRsp | undefined> => {
    return query<BattleHistoryQry, BattleHistoryRsp>(client, arena_address, {
      battle_history: {
        address,
        viewing_key: viewingKey,
        page: paging?.page,
        page_size: paging?.page_size
      }
    })
  }
export const returnFighter = async (client: SigningCosmWasmClient): Promise<WithdrawRsp | undefined> => {
  return transact<WithdrawMsg, WithdrawRsp>(client, arena_address, {
    chicken_out: {

    }
  }, undefined, getFees('pull'))
}

export const getLeaderboards = async (client: CosmWasmClient): Promise<GetLeaderboardsRsp | undefined> => {
  return query<GetLeaderboardsQry, GetLeaderboardsRsp>(client, arena_address, { leaderboards: {} })
}

export const setViewingKey =
  async (client: SigningCosmWasmClient, viewingKey: string, contract: 'area' | 'token'): Promise<ViewingKeyRsp | undefined> => {
    return transact<SetViewingkeyMsg, ViewingKeyRsp>(client, contract === 'area' ? arena_address : card_address, {
      set_viewing_key: {
        key: viewingKey,
        padding: createPadding(viewingKey.length, 256)
      }
    }, undefined, getFees('viewingKeys'))
  }

export const sendHero = async (client: SigningCosmWasmClient, token_id: string): Promise<SendHeroRsp | undefined> => {
  return transact<SendHeroMsg, SendHeroRsp>(client, card_address, {
    send_nft: {
      contract: arena_address,
      token_id,
      msg: btoa(randomString(15))
    }
  }, undefined, getFees('send'))
}

export const getAllTokens = async (client: CosmWasmClient, address: HumanAddr, viewingKey: string):
  Promise<GetAllTokensRsp | undefined> => {
  return query<GetAllTokensQry, GetAllTokensRsp>(client, card_address, {
    tokens: {
      owner: address,
      viewing_key: viewingKey
    }
  })
}

export const getTokenInfo =
  async (client: CosmWasmClient, tokenId: string, address: HumanAddr, viewingKey: string): Promise<GetTokenInfoRsp | undefined> => {
    return query<GetTokenInfoQry, GetTokenInfoRsp>(client, card_address, {
      private_metadata: {
        token_id: tokenId,
        viewer: {
          address,
          viewing_key: viewingKey
        }
      }
    })

  }

export const Contracts = {
  helper: {
    getEntropy,
    createPadding,
    getRandomString: randomString
  },
  minter: {
    address: minter_address,
    mint: mintHeroes
  },

  arena: {
    address: arena_address,
    fightStatusQuery: fightStatus,
    fightHistoryQuery: fightHistory,
    leaderboardsQuery: getLeaderboards,
    returnFigher: returnFighter,
    setViewingKey: (client: SigningCosmWasmClient, viewingKey: string): Promise<ViewingKeyRsp | undefined> =>
      setViewingKey(client, viewingKey, 'area')
  },
  cards: {
    address: card_address,
    setViewingKey: (client: SigningCosmWasmClient, viewingKey: string): Promise<ViewingKeyRsp | undefined> =>
      setViewingKey(client, viewingKey, 'token'),
    sendHero,
    getAllTokens,
    getTokenInfo
  }
}
