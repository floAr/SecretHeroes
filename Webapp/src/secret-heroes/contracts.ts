/* eslint-disable @typescript-eslint/camelcase */

import { toast } from "react-toastify"
// eslint-disable-next-line prettier/prettier
import type { SigningCosmWasmClient } from "secretjs"
// eslint-disable-next-line import/no-unresolved
import { Coin } from "secretjs/types/types"


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
  winning_skill_value: number
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

async function query<IN extends object, OUT>(client: SigningCosmWasmClient, address: HumanAddr, query_msg: IN): Promise<OUT> {
  try {
    const resp = await client.queryContractSmart(address, query_msg)
    return resp
  }
  catch (err) {
    toast.error(`Error running query: ${err}`)
  }
  const resp = await client.queryContractSmart(address, query_msg)
  return resp
}

async function transact<IN extends object, OUT>(client: SigningCosmWasmClient, address: HumanAddr, msg: IN, fee?: Coin): Promise<OUT> {
  let resp
  if (fee !== undefined)
    resp = (await client.execute(address, msg, '', [fee]))
  else
    resp = (await client.execute(address, msg))
  return JSON.parse(new TextDecoder().decode(resp.data)) as OUT
}

const minter_address = "secret1wepvw2r9sdfddu528j3p4lh0v6t46l6c43h43e" as HumanAddr
const arena_address = "secret1tnseuvhc752k4y6tpqm3fng66zgz6w2435ylt9" as HumanAddr
const card_address = "secret13s6fpyw95rtxkvy38dmaqdxc9le86ntev33eye" as HumanAddr

export const mintHeroes = async (client: SigningCosmWasmClient, names: string[]): Promise<MintRsp> => {
  return transact<MintMsg, MintRsp>(client, minter_address, {
    mint: {
      names
    }
  }, {
    denom: 'uscrt', amount: "1000000"
  })
}

export const fightStatus = async (client: SigningCosmWasmClient, address: HumanAddr, viewingKey: string): Promise<FightStatusRsp> => {
  return query<FightStatusQry, FightStatusRsp>(client, arena_address, {
    bullpen: {
      address,
      viewing_key: viewingKey
    }
  })
}
export const fightHistory =
  async (client: SigningCosmWasmClient, address: HumanAddr, viewingKey: string, paging?: { page: number, page_size: number }):
    Promise<BattleHistoryRsp> => {
    return query<BattleHistoryQry, BattleHistoryRsp>(client, arena_address, {
      battle_history: {
        address,
        viewing_key: viewingKey,
        page: paging?.page,
        page_size: paging?.page_size
      }
    })
  }
export const returnFigher = async (client: SigningCosmWasmClient): Promise<WithdrawRsp> => {
  return transact<WithdrawMsg, WithdrawRsp>(client, arena_address, {
    chicken_out: {

    }
  })
}
export const setViewingKey =
  async (client: SigningCosmWasmClient, viewingKey: string, contract: 'area' | 'token'): Promise<ViewingKeyRsp> => {
    return transact<SetViewingkeyMsg, ViewingKeyRsp>(client, contract === 'area' ? arena_address : card_address, {
      set_viewing_key: {
        key: viewingKey,
        padding: createPadding(viewingKey.length, 256)
      }
    })
  }

export const sendHero = async (client: SigningCosmWasmClient, token_id: string): Promise<SendHeroRsp> => {
  return transact<SendHeroMsg, SendHeroRsp>(client, card_address, {
    send_nft: {
      contract: arena_address,
      token_id,
      msg: btoa(randomString(15))
    }
  })
}

export const getAllTokens = async (client: SigningCosmWasmClient, address: HumanAddr, viewingKey: string): Promise<GetAllTokensRsp> => {
  return query<GetAllTokensQry, GetAllTokensRsp>(client, card_address, {
    tokens: {
      owner: address,
      viewing_key: viewingKey
    }
  })
}

export const getTokenInfo =
  async (client: SigningCosmWasmClient, tokenId: string, address: HumanAddr, viewingKey: string): Promise<GetTokenInfoRsp> => {
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
    returnFigher,
    setViewingKey: (client: SigningCosmWasmClient, viewingKey: string): Promise<ViewingKeyRsp> =>
      setViewingKey(client, viewingKey, 'area')
  },
  cards: {
    address: card_address,
    setViewingKey: (client: SigningCosmWasmClient, viewingKey: string): Promise<ViewingKeyRsp> =>
      setViewingKey(client, viewingKey, 'token'),
    sendHero,
    getAllTokens,
    getTokenInfo
  }
}
