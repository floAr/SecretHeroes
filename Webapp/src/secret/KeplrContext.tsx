import React, { useCallback, useContext, useEffect, useReducer, useState } from 'react';
import { toast } from 'react-toastify';
// eslint-disable-next-line prettier/prettier
import type { SigningCosmWasmClient, Account, CosmWasmClient } from 'secretjs';
import { Contracts } from '../secret-heroes/contracts';
import ViewingKeysContext, { viewingKeyContext } from './ViewingKeysContext';


declare global {
  interface Window {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    keplr: any,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    getOfflineSigner: any,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    getEnigmaUtils: any,
    cosmJS: SigningCosmWasmClient,
    Contracts: typeof Contracts
  }
}

export type ChainId = 'secret-4' | 'holodeck-2'
export interface KeplrState {
  keplrFound: boolean
  chainId: string,
  connected: boolean,
  status?: 'undefined' | 'working' | 'idle' | 'failure'
  account?: Account,
  client?: SigningCosmWasmClient,
  queryClient: CosmWasmClient
}

export interface KeplrContextProps extends KeplrState {
  connectQuery: (chainId: ChainId) => Promise<void>,
  connect?: (chainId: ChainId) => void,
  resetViewingKey?: () => void
  viewingKey?: string,
  setWorking?: (running: boolean) => void,
  getQueryClient: () => CosmWasmClient | undefined
}

export type KeplrReducerActions =
  { type: 'connect', chainId: ChainId, queryClient: CosmWasmClient } |
  { type: 'init', chainId: ChainId } |
  {
    type: 'connected', account: Account, client: SigningCosmWasmClient
  } |
  { type: 'error', errorMsg: string } |
  { type: 'transact' } |
  { type: 'success' }

function reducer(state: KeplrState, action: KeplrReducerActions) {
  switch (action.type) {
    case 'connect': {
      return {
        ...state,
        chainId: state.chainId,
        connected: false,
        queryClient: action.queryClient
      } as KeplrState;
    }
    case 'init':
      return {
        chainId: action.chainId,
        connected: false,
        status: 'working'
      } as KeplrState;
    case 'connected':
      return {
        chainId: state.chainId,
        connected: true,
        account: action.account,
        client: action.client, status: 'idle'
      } as KeplrState;
    case 'error':
      return {
        chainId: state.chainId,
        connected: state.connected,
        status: 'failure'
      } as KeplrState;
    case 'transact':
      return {
        ...state,
        status: 'working'
      } as KeplrState
    case 'success':
      return {
        ...state,
        status: 'idle'
      } as KeplrState
    default:
      throw new Error();
  }
}

const intial = { chainId: '', connected: false, status: 'undefined', keplrFound: false, } as KeplrState

const keplrContext = React.createContext<KeplrContextProps>(
  {
    ...intial,
    connectQuery: (_: ChainId) => {
      return new Promise<void>((resolve, reject) => {
        // do nothing
      })
    },
    getQueryClient: () => { return undefined }
  }
)

interface KeplrContextProviderProps {
  children: React.ReactNode,

}

const KeplrContextProvider: React.FC<KeplrContextProviderProps> = ({ children, }) => {

  const isBrowser = typeof window !== 'undefined'
  if (isBrowser) {
    intial.keplrFound = window.keplr !== undefined

  }
  const [keplrState, dispatch] = useReducer(reducer, intial);
  const { getViewingKey } = useContext(viewingKeyContext)

  let viewingKey: string | undefined
  useEffect(() => {
    if (isBrowser && keplrState.account?.address) {
      console.log("loading viewing key for", keplrState.account?.address)
      viewingKey = getViewingKey(keplrState.account.address)

      console.log("loaded", viewingKey)
    }

  }, [keplrState])

  const resetViewingKey = async () => {
    if (keplrState.client != null) {
      dispatch({ type: 'transact' })
      const newKey = Contracts.helper.getEntropy()
      const respArena = await Contracts.arena.setViewingKey(keplrState.client, newKey)
      if (!(respArena.viewing_key.key === newKey)) {
        toast.error("Error setting viewing key for arena contract")
      }
      const respCards = await Contracts.cards.setViewingKey(keplrState.client, newKey)
      if (!(respCards.viewing_key.key === newKey)) {
        toast.error("Error setting viewing key for card contract")
      }
      localStorage.setItem('viewingKey', newKey)
      // setViewingKey(newKey)
      dispatch({ type: 'success' })
      toast.info(`Success: Viewing Key set to ${viewingKey}`)
    }
  }

  const connect = async (chainId: ChainId) => {
    if (!window.getOfflineSigner || !window.keplr) {
      toast.error('Please install and authorize Keplr browser extension')
      return;
    }
    // eslint-disable-next-line no-shadow
    const { CosmWasmClient } = await import("secretjs")

    const queryC: CosmWasmClient = new CosmWasmClient(
      chainId === 'holodeck-2' ? 'https://datahubnode.azurewebsites.net/testnet/' : 'https://datahubnode.azurewebsites.net/node')
    dispatch({ type: 'connect', chainId, queryClient: queryC })
  }

  const setBrowserProvider = async (chainId: ChainId) => {
    if (!window.getOfflineSigner || !window.keplr) {
      toast.error('Please install and authorize Keplr browser extension')
      return;
    }
    dispatch({ type: 'init', chainId })
    if (chainId === 'holodeck-2') {
      await window.keplr.experimentalSuggestChain({
        chainId,
        chainName: 'Secret Testnet',
        rpc: 'http://bootstrap.secrettestnet.io:26657',
        rest: 'https://bootstrap.secrettestnet.io',
        bip44: {
          coinType: 529,
        },
        coinType: 529,
        stakeCurrency: {
          coinDenom: 'SCRT',
          coinMinimalDenom: 'uscrt',
          coinDecimals: 6,
        },
        bech32Config: {
          bech32PrefixAccAddr: 'secret',
          bech32PrefixAccPub: 'secretpub',
          bech32PrefixValAddr: 'secretvaloper',
          bech32PrefixValPub: 'secretvaloperpub',
          bech32PrefixConsAddr: 'secretvalcons',
          bech32PrefixConsPub: 'secretvalconspub',
        },
        currencies: [
          {
            coinDenom: 'SCRT',
            coinMinimalDenom: 'uscrt',
            coinDecimals: 6,
          },
        ],
        feeCurrencies: [
          {
            coinDenom: 'SCRT',
            coinMinimalDenom: 'uscrt',
            coinDecimals: 6,
          },
        ],
        gasPriceStep: {
          low: 0.1,
          average: 0.25,
          high: 0.4,
        },
        features: ['secretwasm'],
      });

    }
    window.keplr.enable(chainId).then(async () => {
      const keplrOfflineSigner = window.getOfflineSigner(chainId);
      const accounts = await keplrOfflineSigner.getAccounts();
      const { address } = accounts[0];
      // eslint-disable-next-line no-shadow
      const { SigningCosmWasmClient } = await import("secretjs")

      const cosmJS: SigningCosmWasmClient = new SigningCosmWasmClient(
        chainId === 'holodeck-2' ? 'https://datahubnode.azurewebsites.net/testnet/' : 'https://datahubnode.azurewebsites.net/node',
        // 'https://bootstrap.secrettestnet.io',
        address,
        keplrOfflineSigner,
        window.getEnigmaUtils(chainId),
        {

        },
      );
      const account = await cosmJS.getAccount(address);

      window.cosmJS = cosmJS
      window.Contracts = Contracts
      if (account !== undefined) {
        toast.success("Connected succesfully")
        dispatch({
          type: 'connected',
          account,
          client: cosmJS
          // chainId,
          // connected: true,
          // account: account,
          // client: cosmJS,
          // status: 'connected'
        })
        if (viewingKey === undefined)
          await resetViewingKey()
      }
    }).catch((e: string) => {
      toast.error(`Error connecting: ${e}`)
      dispatch({
        type: 'error',
        errorMsg: e
      })
    })

  }



  const setWorking = (running: boolean) => {
    if (running)
      dispatch({ type: 'transact' })
    else {
      dispatch({ type: 'success' })
    }

  }

  const getQueryClient = useCallback(
    () => {
      if (keplrState.client != null) return keplrState.client
      return keplrState.queryClient
    },
    [keplrState.queryClient, keplrState.client],
  )

  return <keplrContext.Provider value={
    {
      ...keplrState,
      connect: setBrowserProvider,
      resetViewingKey,
      viewingKey,
      setWorking,
      connectQuery: connect,
      getQueryClient
    }
  }>
    {children}
  </keplrContext.Provider>
}


export { KeplrContextProvider, keplrContext as KeplrContext }
