/* eslint-disable @typescript-eslint/camelcase */
import { css } from '@emotion/core'
import { navigate } from 'gatsby'
import * as React from 'react'
import { useEffect, useState } from 'react'
import { toast } from 'react-toastify'
import BattleReportFrame from '../components/BattleReport/BattleReportFrame'

// import Modal from 'react-modal'
import UnityFunc from '../components/UnityFunc'
import ViewingKeyController from '../components/ViewingKeyController'
import IndexLayout from '../layouts'
import { Battle, Contracts, getEntropy } from '../secret-heroes/contracts'
import { KeplrContext } from '../secret/KeplrContext'
import { viewingKeyContext } from '../secret/ViewingKeysContext'

declare global {
  interface Window {
    mint: () => Promise<void>
    poll: () => Promise<void>
    sendToBattle: (tokenId: string) => Promise<void>
    registerUnityInstance: React.Dispatch<React.SetStateAction<UnityInstance | undefined>>
  }
}

interface Token {
  id: string
  name: string
  weapons: number
  engineering: number
  biotech: number
  psychics: number
  base_weapons: number
  base_engineering: number
  base_biotech: number
  base_psychics: number
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

  const { getViewingKey } = React.useContext(viewingKeyContext)
  const [viewingKeyValid, setViewingKeyValid] = React.useState<boolean>(false)
  const { connected, account, client, setWorking } = React.useContext(KeplrContext)
  const [unityInstance, setUnityInstance] = React.useState<UnityInstance | undefined>(undefined)

  const [battleState, setBattleState] = useState<BattleState | undefined>(undefined)
  const [battleHistory, setBattleHistory] = useState<Battle[]>([])
  const [registeredTokens, setRegisteredTokens] = useState<string[]>([])

  const permToastId = React.useRef<string | null | number>(null)

  const [viewingKey, setViewingKey] = useState<string | undefined>(undefined)
  useEffect(() => {
    if (isBrowser && account?.address) setViewingKey(getViewingKey(account?.address))
  }, [account, getViewingKey])

  useEffect(() => {
    if (!connected) {
      navigate('/connect')
    }
  }, [connected])

  if (isBrowser) window.registerUnityInstance = setUnityInstance

  // update the historic battles
  const PollBattleHistory = async () => {
    if (!viewingKeyValid) return null
    if (client != null && account?.address != null && viewingKey != null) {
      const newBattleHistory = await Contracts.arena.fightHistoryQuery(client, account.address, viewingKey)
      if (newBattleHistory && JSON.stringify(newBattleHistory) !== JSON.stringify(battleHistory))
        setBattleHistory(newBattleHistory?.battle_history.history)
      return true
    }
    return null
  }

  async function PollFightState() {
    if (!viewingKeyValid) return null
    if (client != null && account?.address != null && viewingKey != null) {
      const fightState = await Contracts.arena.fightStatusQuery(client, account?.address, viewingKey)
      if (fightState) {
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
            psychics: fightState.bullpen.your_hero?.stats.current[3],
            base_weapons: fightState.bullpen.your_hero?.stats.base[0],
            base_engineering: fightState.bullpen.your_hero?.stats.base[1],
            base_biotech: fightState.bullpen.your_hero?.stats.base[2],
            base_psychics: fightState.bullpen.your_hero?.stats.base[3]
          }
        }

        switch (newBattleState.heroes_waiting) {
          case 0:
            toast.info('The arena is empty')
            break
          case 1:
            toast.info(newBattleState.your_hero != null ? 'You are waiting in the arena alone' : 'One hero is waiting in the arena')
            break
          case 2:
            toast.info(
              newBattleState.your_hero != null
                ? 'You are waiting in the arena with one hero'
                : 'Two heroes are waiting in the arena for YOU'
            )
            break
          default:
            toast.info('The battle commences')
        }
        if (JSON.stringify(newBattleState) !== JSON.stringify(battleState)) {
          PollBattleHistory()
          if (unityInstance !== undefined) {
            unityInstance.SendMessage('WebGlBridge', 'ReportBattleStatus', JSON.stringify(newBattleState))
            setBattleState(newBattleState)
          }
        }
        return newBattleState
      }
      return null
    }
  }

  const getToken = async (tokenId: string) => {
    if (client != null && account?.address != null && viewingKey != null) {
      const token = await Contracts.cards.getTokenInfo(client, tokenId, account?.address, viewingKey)
      if (token) {
        const image = JSON.parse(token.private_metadata.image) as { base: number[]; current: number[] }
        return {
          id: tokenId,
          name: token.private_metadata.name,
          weapons: image.current[0],
          engineering: image.current[1],
          biotech: image.current[2],
          psychics: image.current[3],
          base_weapons: image.base[0],
          base_engineering: image.base[1],
          base_biotech: image.base[2],
          base_psychics: image.base[3]
        } as Token
      }
    }
    return {} as Token
  }

  const PollNewTokens = async (paginate?: boolean) => {
    if (!viewingKeyValid) return undefined
    const newTokens: Token[] = []
    let hasChanges = false
    if (client != null && account?.address != null && viewingKey != null) {
      const allTokens = await Contracts.cards.getAllTokens(client, account?.address, viewingKey)
      if (allTokens) {
        const tokenIds: string[] = allTokens.token_list.tokens
        hasChanges = JSON.stringify(tokenIds) !== JSON.stringify(registeredTokens)
        console.log(tokenIds)
        if (hasChanges) {
          // eslint-disable-next-line no-restricted-syntax

          for (let i = 0; i < tokenIds.length; i += 1) {
            const tokenId = tokenIds[i]
            if (!registeredTokens.includes(tokenId)) {
              // eslint-disable-next-line no-await-in-loop
              const t = await getToken(tokenId)
              newTokens.push(t)
              if (paginate && i >= 4) {
                break
              }
            }
          }

          if (unityInstance !== undefined && newTokens.length > 0) {
            JSON.stringify(newTokens)
            unityInstance.SendMessage('WebGlBridge', 'ReportTokens', JSON.stringify(newTokens))
            setRegisteredTokens(
              newTokens.map(token => {
                return token.id
              })
            )
          }
        }
      }
    } else {
      setViewingKeyValid(false)
    }
    if (paginate && hasChanges) {
      PollNewTokens()
    }
    return newTokens
  }

  const SendToBattle = async (tokenId: string) => {
    if (client != null && setWorking != null) {
      try {
        setWorking(true)
        await Contracts.cards.sendHero(client, tokenId)
        await PollFightState()
        setWorking(false)
      } catch (e) {
        toast.error(`Error sending hero to battle: ${e}`)
      }
    }
  }

  const ReturnFighter = async () => {
    if (client != null && setWorking != null) {
      setWorking(true)
      await Contracts.arena.returnFigher(client)
      setWorking(false)
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

    if (client != null && setWorking != null) {
      setWorking(true)
      const mint = await Contracts.minter.mint(client, [name1, name2, name3])

      if (mint && mint.mint.status === 'Success') {
        let newTokens = await PollNewTokens()
        newTokens = newTokens?.sort((t1, t2) => (Number(t1.id) < Number(t2.id) ? 1 : -1)).slice(0, 3)
        if (unityInstance !== undefined) {
          unityInstance.SendMessage('WebGlBridge', 'RegisterMint', JSON.stringify(newTokens))
        }
      }
      setWorking(false)
    }
  }

  const PollAll = async () => {
    if (setWorking != null) {
      setWorking(true)
      permToastId.current = toast.info('Assembling your heroes', { closeButton: false, autoClose: false, closeOnClick: true })
      const pollResult = await PollNewTokens(true)
      if (pollResult == null) {
        toast.dismiss(permToastId.current)
        return false
      }
      toast.dismiss(permToastId.current)
      permToastId.current = toast.info('Investigating previous battles', { closeButton: false, autoClose: false, closeOnClick: true })
      const historyResult = await PollBattleHistory()
      if (historyResult == null) {
        toast.dismiss(permToastId.current)
        return false
      }
      toast.dismiss(permToastId.current)
      permToastId.current = toast.info('Checking for current battles', { closeButton: false, autoClose: false, closeOnClick: true })
      const stateResult = await PollFightState()
      if (stateResult == null) {
        toast.dismiss(permToastId.current)
        return false
      }
      toast.dismiss(permToastId.current)
      setWorking(false)
      return true
    }
    return false
  }

  if (isBrowser) {
    window.mint = MintHeroes
    window.poll = PollFightState
    window.sendToBattle = SendToBattle
  }

  useEffect(() => {
    if (unityInstance !== undefined && connected && client && account && viewingKey && viewingKeyValid) {
      Contracts.cards
        .getAllTokens(client, account?.address, viewingKey)
        .then(async () => {
          if (await PollAll()) unityInstance.SendMessage('WebGlBridge', 'Connect')
        })
        .catch(async _ => {
          console.log('viewingKeys are out of sync')
        })
    }
  }, [connected, unityInstance, viewingKey, viewingKeyValid])

  useEffect(() => {
    const getFightState = setInterval(async () => {
      await PollFightState()
    }, 30000)

    // clearing interval
    return () => clearInterval(getFightState)
  })

  return (
    <div
      css={css`
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: start;
        margin-top: 100px;
        padding: 0 24px 24px 24px;
      `}
    >
      <div
        css={css`
          display: contents;
        `}
      >
        {connected && <UnityFunc />}
        {/* <Modal
        isOpen={modalIsOpen}
        // onAfterOpen={afterOpenModal}
        // onRequestClose={closeModal}
        // style={customStyles}
        contentLabel="Mint Heroes"
        >
        Mint heroes
      </Modal> */}
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: column;
          width: 100%;
          min-width: 700px;
          flex: 1 1 500px;
          @media screen and (min-width: 1800px) {
            padding: 0 0 0 24px;
          }
        `}
      >
        {viewingKeyValid ? (
          <>
            <h3
              css={css`
                color: white;
                margin-bottom: 24px;
              `}
            >
              Battle History
            </h3>

            <BattleReportFrame battles={battleHistory} />
          </>
        ) : (
          <div
            css={css`
              max-width: 400px;
            `}
          >
            <ViewingKeyController setValid={setViewingKeyValid} />
          </div>
        )}
      </div>
      {/* <div
        css={css`
          display: flex;
          flex-direction: column;
          width: 100%;
          min-width: 700px;
          flex: 1 1 500px;
        `}
      >
        <h3
          css={css`
            padding-left: 40px;
          `}
        >
          Battle Results
        </h3>

        <BattleStateFrame battles={battleHistory} />
      </div> */}
    </div>
  )
}

const GamePage = () => (
  <IndexLayout>
    <Game />
  </IndexLayout>
)

export default GamePage
