import { css } from '@emotion/core'
import { graphql, useStaticQuery } from 'gatsby'
import * as React from 'react'
import { useEffect, useState } from 'react'
import { SigningCosmWasmClient } from 'secretjs'
import IndexLayout from '../layouts'
import Img from 'gatsby-image'

import contracts from '../secretReact/contracts'
import { useInterval } from '../secretReact/useInterval'
import BattleReportRender from '../components/BattleReport'

import UnityFunc from '../components/UnityFunc'
import BattleStateRender from '../components/BattleState'

const chainId = 'holodeck-2'

const buildUrl = 'Build'



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

interface PollingData {
  tokens: Token[]
}

const IndexPage = () => {
  const data = useStaticQuery(graphql`
    query {
      file(relativePath: { eq: "secret-heroes.png" }) {
        childImageSharp {
          # Specify a fixed image and fragment.
          # The default width is 400 pixels
          fixed(width: 150) {
            ...GatsbyImageSharpFixed
          }
          fluid {
            ...GatsbyImageSharpFluid
          }
        }
      }
    }
  `)
  const isBrowser = typeof window !== 'undefined'

  const [keplrReady, setKeplrReady] = useState<boolean>(false)
  const [account, setAccount] = useState<Account | undefined>(undefined)
  const [client, setClient] = useState<SigningCosmWasmClient | undefined>(undefined)
  const [viewingKey, setViewingKey] = useState<string | null>(isBrowser ? localStorage.getItem('viewingKey') : null)
  const [unityInstance, setUnityInstance] = useState(undefined)
  const [battleHistory, setBattleHistory] = useState<BattleResult[]>([])
  const [battleState, setBattleState] = useState<BattleState | undefined>(undefined)

  if (isBrowser) window.registerUnityInstance = setUnityInstance

  //#region Function to connect
  const setupKeplr = async (chainId: string) => {
    // Define sleep
    const sleep = (ms: number) => new Promise(accept => setTimeout(accept, ms))

    // Wait for Keplr to be injected to the page
    while (!window.keplr && !window.getOfflineSigner && !window.getEnigmaUtils) {
      await sleep(10)
    }

    await window.keplr.experimentalSuggestChain({
      chainId,
      chainName: 'Secret Testnet',
      rpc: 'http://bootstrap.secrettestnet.io:26657',
      rest: 'https://bootstrap.secrettestnet.io',
      bip44: {
        coinType: 529
      },
      coinType: 529,
      stakeCurrency: {
        coinDenom: 'SCRT',
        coinMinimalDenom: 'uscrt',
        coinDecimals: 6
      },
      bech32Config: {
        bech32PrefixAccAddr: 'secret',
        bech32PrefixAccPub: 'secretpub',
        bech32PrefixValAddr: 'secretvaloper',
        bech32PrefixValPub: 'secretvaloperpub',
        bech32PrefixConsAddr: 'secretvalcons',
        bech32PrefixConsPub: 'secretvalconspub'
      },
      currencies: [
        {
          coinDenom: 'SCRT',
          coinMinimalDenom: 'uscrt',
          coinDecimals: 6
        }
      ],
      feeCurrencies: [
        {
          coinDenom: 'SCRT',
          coinMinimalDenom: 'uscrt',
          coinDecimals: 6
        }
      ],
      gasPriceStep: {
        low: 0.1,
        average: 0.25,
        high: 0.4
      },
      features: ['secretwasm']
    })

    // Enable Keplr.
    // This pops-up a window for the user to allow keplr access to the webpage.
    await window.keplr.enable(chainId)
    setKeplrReady(true)
  }

  const setupClient = async () => {
    // Setup SecrtJS with Keplr's OfflineSigner
    // This pops-up a window for the user to sign on each tx we sent
    const keplrOfflineSigner = window.getOfflineSigner(chainId)
    const accounts = await keplrOfflineSigner.getAccounts()

    const client = new SigningCosmWasmClient(
      'https://bootstrap.secrettestnet.io', // holodeck - https://bootstrap.secrettestnet.io; mainnet - user your LCD/REST provider
      accounts[0].address,
      keplrOfflineSigner,
      window.getEnigmaUtils(chainId),
      {
        // 300k - Max gas units we're willing to use for init
        init: {
          amount: [{ amount: '300000', denom: 'uscrt' }],
          gas: '300000'
        },
        // 300k - Max gas units we're willing to use for exec
        exec: {
          amount: [{ amount: '980000', denom: 'uscrt' }],
          gas: '980000'
        }
      }
    )
    if (isBrowser) window.scrtClient = client
    setClient(client)
  }

  // const query = async (address: string, msg: any) => {
  //   const response = await client?.queryContractSmart(address, msg)
  //   console.log(response)
  //   const decoded = new TextDecoder().decode(response.data)
  //   console.log('query: ', decoded)
  //   return JSON.parse(decoded)
  // }

  const execute = async (address: string, msg: any) => {
    const response = await client?.execute(address, msg)
    const decoded = new TextDecoder().decode(response?.data)
    console.log('exec: ', decoded)
    return JSON.parse(decoded)
  }

  const getToken = async (tokenId: string) => {
    const privateData = await client?.queryContractSmart(
      contracts.nft.address,
      contracts.nft.queries.privateMetadata(tokenId, account.address)
    )
    const skills: number[] = JSON.parse(privateData.private_metadata.image)
    const token: Token = {
      id: tokenId,
      name: privateData.private_metadata.name,
      weapons: skills[0],
      engineering: skills[1],
      biotech: skills[2],
      psychics: skills[3]
    }
    return token
  }

  const pollFightState = async () => {
    const fightState = await client?.queryContractSmart(contracts.bullpen.address, contracts.bullpen.queries.fightStatus(account.address))
    // eslint-disable-next-line @typescript-eslint/camelcase
    const _battleState: BattleState = { heroes_waiting: fightState.bullpen.heroes_waiting, your_hero: null }
    if (fightState.bullpen.your_hero !== null) {

      // eslint-disable-next-line @typescript-eslint/camelcase
      _battleState.your_hero = {
        id: fightState.bullpen.your_hero.name,
        name: fightState.bullpen.your_hero.name,
        weapons: fightState.bullpen.your_hero.skills[0],
        engineering: fightState.bullpen.your_hero.skills[1],
        biotech: fightState.bullpen.your_hero.skills[2],
        psychics: fightState.bullpen.your_hero.skills[3]
      }
      if (JSON.stringify(_battleState) !== JSON.stringify(battleState)) {
        if (unityInstance !== undefined) {
          unityInstance.SendMessage('WebGlBridge', 'ReportBattleStatus', JSON.stringify(_battleState))
          setBattleState(_battleState)
        }
      }
    }

  }

  const pollData = async () => {
    const tokens: Token[] = []
    const allTokens = await client?.queryContractSmart(
      contracts.nft.address,
      contracts.nft.queries.getAllTokens(window.scrtAccount.address)
    )
    const tokenIds: string[] = allTokens.token_list.tokens

    // eslint-disable-next-line no-restricted-syntax
    for (const tokenId of tokenIds) {
      // eslint-disable-next-line no-await-in-loop
      tokens.push(await getToken(tokenId))
    }
    if (unityInstance !== undefined && tokens.length > 0) {
      unityInstance.SendMessage('WebGlBridge', 'ReportTokens', JSON.stringify(tokens))
    }
  }

  const pollBattleHistory = async () => {
    const tokens: Token[] = []
    const battleHistory = await client?.queryContractSmart(
      contracts.bullpen.address,
      contracts.bullpen.queries.fightHistory(account.address)
    )
    console.log(battleHistory)
    if (JSON.stringify(battleHistory) !== JSON.stringify(battleHistory.battle_history.history))
      setBattleHistory(battleHistory.battle_history.history as BattleResult[])
  }

  const returnFigher = async () => {
    await client?.execute(contracts.bullpen.address, contracts.bullpen.messages.returnFigher())
  }

  const setupAccount = async () => {
    const scrtAccount = await client?.getAccount(account?.address)
    if (viewingKey === undefined || viewingKey === null) {
      const vKey = await execute(contracts.bullpen.address, contracts.bullpen.messages.setViewingKey())
      await execute(contracts.nft.address, contracts.nft.messages.setViewingKey())
      try {
        localStorage.setItem('viewingKey', vKey.viewing_key.key)
        setViewingKey(vKey.viewing_key.key)
      } catch (error) { }
    }

    setAccount(scrtAccount)
  }

  const saveName = (name: string | null | undefined) => {
    if (name === undefined || name === null || name.length === 0) return contracts.getEntropy()
    return name
  }

  const registerMinting = (tokens: Token[]) => {
    if (unityInstance !== undefined) {
      unityInstance.SendMessage('WebGlBridge', 'RegisterMint', JSON.stringify(tokens))
    }
  }

  const mintHeroes = async () => {
    const name1 = saveName(prompt('Enter the name of your first hero'))
    const name2 = saveName(prompt('Enter the name of your second hero'))
    const name3 = saveName(prompt('Enter the name of your third hero'))

    const mintResult = await execute(contracts.minter.address, contracts.minter.messages.mint(name1, name2, name3))
    if (mintResult.status.status === 'Success') {
      const mintedTokens = []
      mintedTokens.push(await getToken(name1))
      mintedTokens.push(await getToken(name2))
      mintedTokens.push(await getToken(name3))
      registerMinting(mintedTokens)
    }
  }

  useEffect(() => {
    setupKeplr(chainId)
  }, [])

  useEffect(() => {
    if (keplrReady) {
      setupClient()
    }
  }, [keplrReady])

  useEffect(() => {
    if (client) {
      setupAccount()
    }
  }, [client])

  useEffect(() => {
    console.log('trying unlock - account: ' + account + ' & unityInstace: ' + unityInstance)
    if (account && unityInstance) {
      pollBattleHistory();
      pollData();
      pollFightState();
      unityInstance.SendMessage('WebGlBridge', 'Connect')
    }
  }, [account, unityInstance])

  useInterval(async () => {
    if (account) {
      await pollData()
      await pollFightState()
      await pollBattleHistory()
    }
  }, 15000)

  //#endregion

  const SendToBattle = async (tokenId: string) => {
    try {
      const result = await execute(contracts.nft.address, contracts.nft.messages.sendNft(tokenId))
      console.log(result)
    } catch (e) {
      console.log(e)
      console.log('error')
    }
  }

  if (isBrowser) {
    window.scrtContracts = contracts
    window.scrtAccount = account

    // window.scrtQuery = query
    window.scrtExec = execute

    window.scrtPoll = pollData
    window.scrtPollBattle = pollFightState
    window.scrtMint = mintHeroes
    window.scrtSentToBattle = SendToBattle
  }

  return (
    <IndexLayout>
      <div
        css={css`
          display: grid;
          grid-template-columns: 0.1fr 8fr 2fr 0.1fr;
          grid-template-rows: 0.2fr 3fr;
          grid-column-gap: 5px;
          grid-row-gap: 5px;
          place-items: center;
          height: 100%;
          align-content: stretch;
        `}
      >

        <Img
          css={css`
              grid-area: 1 / 3 / 2 / 4;
              width: 100%;
              height: 100%;
              object-fit:contain;
            `}
          fluid={data.file.childImageSharp.fluid}
          alt="Secret Heroes"
        />


        <div
          css={css`
            grid-area: 1 / 2 / 2 / 3;
            align-content: center;
            align-items: center;
            vertical-align: middle;
            width: 100%;
            font-weight: bold;
            padding: 0.1vw 2vw 0.1vw 2vw;
            border-radius: 10px;
            background: rgb(245, 245, 245);
            backdrop-filter: blur(10px);
            box-shadow: -0.25rem -0.25rem 0.5rem rgba(255, 255, 255, 0.07), 0.25rem 0.25rem 0.5rem rgba(0, 0, 0, 0.12), -0.75rem -0.75rem 1.75rem rgba(255, 255, 255, 0.07), 0.75rem 0.75rem 1.75rem rgba(0, 0, 0, 0.12), inset 8rem 8rem 8rem rgba(0, 0, 0, 0.05), inset -8rem -8rem 8rem rgba(255, 255, 255, 0.05);
            display: flex;
            justify-content: space-between`}
        >
          {account === undefined ? (
            <h4
              css={css`
                margin-left: 15px;
              `}
            >
              Connecting - you need the <a href={'https://wallet.keplr.app/'}> Keplr Wallet</a> to interact with the game
            </h4>
          ) : (
            <h4
              css={css`
                margin-left: 15px;
              `}
            >
              Connected as {account.address}
            </h4>
          )}
          <div css={css`display:flex;flex-direction:row; align-items:center`}>
            <h4 css={css`padding:15px`}>Currently waiting to fight:</h4>
            {battleState && <BattleStateRender report={battleState} />}
            {battleState === undefined && <h4>-</h4>}
          </div>
        </div>
        <div
          css={css`
            grid-area: 2 / 2 / 3 / 3;
            height: 100%;
            width: 100%;
            padding: 5vh 5vw;
            border-radius: 10px;
            background: rgb(245, 245, 245);
            backdrop-filter: blur(10px);
            box-shadow: -0.25rem -0.25rem 0.5rem rgba(255, 255, 255, 0.07), 0.25rem 0.25rem 0.5rem rgba(0, 0, 0, 0.12), -0.75rem -0.75rem 1.75rem rgba(255, 255, 255, 0.07), 0.75rem 0.75rem 1.75rem rgba(0, 0, 0, 0.12), inset 8rem 8rem 8rem rgba(0, 0, 0, 0.05), inset -8rem -8rem 8rem rgba(255, 255, 255, 0.05);

          `}
        >
          <UnityFunc />
          {/* <div css={css`background-color: aliceblue`} /> */}
        </div>
        <div
          css={css`
            grid-area: 2 / 3 / 3 / 4;
            /* background-color: #0f8824; */
            width: 100%;
            height: 100%;
            align-self: flex-start;
            padding: 5vh 5vw;
            border-radius: 10px;
            background: rgb(245, 245, 245);
            backdrop-filter: blur(10px);
            box-shadow: -0.25rem -0.25rem 0.5rem rgba(255, 255, 255, 0.07), 0.25rem 0.25rem 0.5rem rgba(0, 0, 0, 0.12), -0.75rem -0.75rem 1.75rem rgba(255, 255, 255, 0.07), 0.75rem 0.75rem 1.75rem rgba(0, 0, 0, 0.12), inset 8rem 8rem 8rem rgba(0, 0, 0, 0.05), inset -8rem -8rem 8rem rgba(255, 255, 255, 0.05);

          `}
        >
          <h4>Battle Results</h4>
          {battleHistory.length === 0 ? ("No battles yet") : (
            <ul>
              {battleHistory.map(battle => {
                return (
                  <li
                    css={css`
                    width: 100%;
                  `}
                    key={battle.battle_number}
                  >
                    <BattleReportRender report={battle} />{' '}
                  </li>
                )
              })}
            </ul>)}
        </div>
      </div>
    </IndexLayout>
  )
}

export default IndexPage
