import React, { useReducer, useState } from 'react';
// eslint-disable-next-line prettier/prettier
import type { SigningCosmWasmClient, Account  } from 'secretjs';
import { Contracts } from '../secret-heroes/contracts';

declare global {
  interface Window{
    keplr:any,
    getOfflineSigner:any,
    getEnigmaUtils:any,
    cosmJS: SigningCosmWasmClient,
    Contracts: typeof Contracts
  }
}

export type ChainId = 'secret-2'|'holodeck-2'
export interface KeplrState{
  keplrFound:boolean
  chainId: string,
  connected: boolean,
  status?: 'undefined' | 'working' | 'idle' | 'failure'
  account?: Account,
  client?: SigningCosmWasmClient,
}

export interface KeplrContextProps extends KeplrState{
  connect?: (chainId:ChainId)=>void,
  resetViewingKey?: ()=>void
  viewingKey?: string
}

export type KeplrReducerActions={type:'init', chainId:ChainId}|{
  type:'connected',account:Account, client:SigningCosmWasmClient}|
  {type:'error',errorMsg:string}


function reducer(state: KeplrState, action:KeplrReducerActions) {
  switch (action.type) {
   case 'init':
     return {chainId:action.chainId,connected:false} as KeplrState;
   case 'connected':
    return {chainId:state.chainId,
connected:true,
account:action.account,
client:action.client,status:'idle'} as KeplrState;
case 'error':
  return {chainId:state.chainId,
    connected:state.connected,
  status:'failure'}as KeplrState;

    default:
      throw new Error();
  }
}

const intial = {chainId:'',connected:false,status:'undefined',keplrFound:false} as KeplrState

const keplrContext = React.createContext<KeplrContextProps>(intial)

interface KeplrContextProviderProps{
  children: React.ReactNode,

}






const KeplrContextProvider: React.FC<KeplrContextProviderProps> = ({children,})=>{

  const isBrowser = typeof window !== 'undefined'
  if(isBrowser){
intial.keplrFound=window.keplr
  }
  const [keplrState, dispatch] = useReducer(reducer, intial);

  const [viewingKey, setViewingKey] = useState<string | undefined>(isBrowser ? localStorage.getItem('viewingKey') : undefined)


  const setBrowserProvider = async (chainId: ChainId) => {
    if (!window.getOfflineSigner || !window.keplr) throw new Error('Please authorize browser extension (Keplr or similar)')
    dispatch({type:'init',chainId})
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
    const res = window.keplr.enable(chainId).then(async () => {
      const keplrOfflineSigner = window.getOfflineSigner(chainId);
      const accounts = await keplrOfflineSigner.getAccounts();
      const {address} = accounts[0];
       // eslint-disable-next-line no-shadow
       const { SigningCosmWasmClient } = await import("secretjs")

      const cosmJS: SigningCosmWasmClient = new SigningCosmWasmClient(
        chainId==='holodeck-2'?'https://datahubnode.azurewebsites.net/testnet/':'https://datahubnode.azurewebsites.net/node',
        // 'https://bootstrap.secrettestnet.io',
        address,
        keplrOfflineSigner,
        window.getEnigmaUtils(chainId),
        {
          init: {
            amount: [{ amount: '300000', denom: 'uscrt' }],
            gas: '300000',
          },
          exec: {
            amount: [{ amount: '500000', denom: 'uscrt' }],
            gas: '500000',
          },
        },
      );
      const account = await cosmJS.getAccount(address);

      window.cosmJS = cosmJS
      window.Contracts=Contracts
      if(account!==undefined){
      dispatch({
        type:'connected',
        account,
        client:cosmJS
        // chainId,
        // connected: true,
        // account: account,
        // client: cosmJS,
        // status: 'connected'
      })
    }
    }).catch((e:any) => {
      console.log(e)
      // _toast?.error("Error", e)
      dispatch({
       type:'error',
       errorMsg: e
      })
    })

  }

  const resetViewingKey=async ()=>{
    if(keplrState.client!=null){
    const newKey= Contracts.helper.getEntropy()
    const respArena = await Contracts.arena.setViewingKey(keplrState.client,newKey)
    if(!(respArena.viewing_key.key===newKey)){
      // error
    }
    const respCards = await Contracts.cards.setViewingKey(keplrState.client,newKey)
    if(!(respCards.viewing_key.key===newKey)){
      // error
    }
    localStorage.setItem('viewingKey',newKey)
    setViewingKey(newKey)
    }
  }

  return <keplrContext.Provider value={{...keplrState, connect:setBrowserProvider,resetViewingKey,viewingKey}}>{children}</keplrContext.Provider>
}


export  {KeplrContextProvider, keplrContext as KeplrContext}
