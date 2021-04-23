import * as React from 'react'
import { useEffect, useState } from 'react'

import Container from '../components/Container'
import UnityFunc from '../components/UnityFunc'
import IndexLayout from '../layouts'
import { Contracts, getEntropy, HeroStats } from '../secret-heroes/contracts'
import { KeplrContext } from '../secret/KeplrContext'

interface Token {
  id: string
  name: string
  weapons: number
  engineering: number
  biotech: number
  psychics: number
}

export interface BattleResult {
  battle_number: number
  my_hero: string
  my_token_id: string
  my_skills: number[]
  skill_used: number
  winning_skill_value: number
  i_won: boolean
}

export interface BattleState {
  heroes_waiting: number
  your_hero?: Token
}

interface UnityInstance {
  test: boolean
  SendMessage: (targetObject: string, functionName: string, payload?: string) => void
}

const Game = () => {
  const isBrowser = typeof window !== 'undefined'

  const { connected, account, client, viewingKey } = React.useContext(KeplrContext)
  const [unityInstance, setUnityInstance] = React.useState<UnityInstance | undefined>(undefined)

  const [battleState, setBattleState] = useState<BattleState | undefined>(undefined)
  const [registeredTokens, setRegisteredTokens] = useState<string[]>([])

  if (isBrowser) window.registerUnityInstance = setUnityInstance

  const getUpgradeValue = (stats: HeroStats, index: number) => {
    return stats.current[index] - stats.base[index]
  }

  console.log(viewingKey)
  const PollFightState = async () => {
    if (client != null && account?.address != null && viewingKey != null) {
      // const fightState = await client?.queryContractSmart(contracts.bullpen.address, contracts.bullpen.queries.fightStatus(account.address))
      const fightState = await Contracts.arena.fightStatusQuery(client, account?.address, viewingKey)
      // eslint-disable-next-line @typescript-eslint/camelcase
      const newBattleState: BattleState = { heroes_waiting: fightState.bullpen.heroes_waiting, your_hero: undefined }
      if (fightState.bullpen.your_hero != null && fightState.bullpen.your_hero?.stats != null) {
        // eslint-disable-next-line @typescript-eslint/camelcase
        newBattleState.your_hero = {
          id: fightState.bullpen.your_hero?.name,
          name: fightState.bullpen.your_hero?.name,
          weapons: fightState.bullpen.your_hero?.stats.current[0],
          engineering: fightState.bullpen.your_hero?.stats.current[1],
          biotech: fightState.bullpen.your_hero?.stats.current[2],
          psychics: fightState.bullpen.your_hero?.stats.current[3]
        }
        if (JSON.stringify(newBattleState) !== JSON.stringify(battleState)) {
          if (unityInstance !== undefined) {
            unityInstance.SendMessage('WebGlBridge', 'ReportBattleStatus', JSON.stringify(newBattleState))
            setBattleState(newBattleState)
          }
        }
      }
    }
  }

  const PollBattleHistory = async () => {
    if (client != null && account?.address != null && viewingKey != null) {
      const battleHistory = await Contracts.arena.fightHistoryQuery(client, account.address, viewingKey)
      console.log(battleHistory)
    }
  }

  const getToken = async (tokenId: string) => {
    if (client != null && account?.address != null && viewingKey != null) {
      const token = await Contracts.cards.getTokenInfo(client, tokenId, account?.address, viewingKey)
      const image = JSON.parse(token.private_metadata.image) as { base: number[]; current: number[] }
      console.log('token ', token, 'image ', image)
      return {
        id: tokenId,
        name: token.private_metadata.name,
        weapons: image.current[0],
        engineering: image.current[1],
        biotech: image.current[2],
        psychics: image.current[3]
      } as Token
    }
    return {} as Token
  }

  const PollNewTokens = async () => {
    const newTokens: Token[] = []
    if (client != null && account?.address != null && viewingKey != null) {
      const allTokens = await Contracts.cards.getAllTokens(client, account?.address, viewingKey)
      const tokenIds: string[] = allTokens.token_list.tokens
      console.log('old ', registeredTokens, ' new ', tokenIds)
      if (JSON.stringify(tokenIds) !== JSON.stringify(registeredTokens)) {
        // eslint-disable-next-line no-restricted-syntax
        console.log(tokenIds)
        for (let i = 0; i < tokenIds.length; i++) {
          const tokenId = tokenIds[i]
          console.log(tokenId)
          if (!registeredTokens.includes(tokenId)) {
            console.log('querying ', tokenId)
            // eslint-disable-next-line no-await-in-loop
            const t = await getToken(tokenId)
            newTokens.push(t)
          }
        }
        // await tokenIds.forEach(async tokenId => {})
        console.log('registering', newTokens)
        if (unityInstance !== undefined && newTokens.length > 0) {
          console.log('registered', newTokens)
          unityInstance.SendMessage('WebGlBridge', 'ReportTokens', JSON.stringify(newTokens))
          setRegisteredTokens(tokenIds)
        }
      }
    }
    return newTokens
  }

  const SendToBattle = async (tokenId: string) => {
    if (client != null) {
      try {
        await Contracts.cards.sendHero(client, tokenId)
        await PollFightState()
      } catch (e) {
        console.log(e)
      }
    }
  }

  const ReturnFighter = async () => {
    if (client != null) {
      await Contracts.arena.returnFigher(client)
    }
  }

  const getSaveName = (name: string | null | undefined) => {
    if (name === undefined || name === null || name.length === 0) return getEntropy()
    return name
  }

  const MintHeroes = async () => {
    const name1 = getSaveName(prompt('Enter the name of your first hero'))
    const name2 = getSaveName(prompt('Enter the name of your second hero'))
    const name3 = getSaveName(prompt('Enter the name of your third hero'))

    if (client != null) {
      const mint = await Contracts.minter.mint(client, [name1, name2, name3])
      console.log(mint)
      if (mint.mint.status === 'Success') {
        let newTokens = await PollNewTokens()
        newTokens = newTokens.sort((t1, t2) => (Number(t1.id) < Number(t2.id) ? 1 : -1)).slice(0, 3)
        console.log('new from mint', newTokens)
        if (unityInstance !== undefined) {
          unityInstance.SendMessage('WebGlBridge', 'RegisterMint', JSON.stringify(newTokens))
        }
      }
    }
  }

  const PollAll = async () => {
    await PollNewTokens()
    await PollBattleHistory()
    await PollFightState()
  }
  if (isBrowser) {
    window.mint = MintHeroes
    window.poll = PollAll
    window.sendToBattle = SendToBattle
  }

  useEffect(() => {
    console.log('Connection status: ', connected, ' - Unity Instance: ', unityInstance)
    if (unityInstance !== undefined && connected) {
      unityInstance.SendMessage('WebGlBridge', 'Connect')
      PollNewTokens()
    }
  }, [connected, unityInstance])

  return (
    <div>
      <UnityFunc />
    </div>
  )
}

const GamePage = () => (
  <IndexLayout>
    <Container>
      <Game />
    </Container>
  </IndexLayout>
)

export default GamePage
