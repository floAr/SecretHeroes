// import { useState } from "react"
// import { SigningCosmWasmClient } from "secretjs"

// interface KeplrConnectionProps{
//   network: NetworkConfig
// }


// const KeplrConnection: React.FC<KeplrConnectionProps> = ({network}) => {
//   const isBrowser = typeof window !== 'undefined'

//   const [keplrEnabled,setKeplkEnabled] = useState<boolean>(false)

//   // wait for the browser injected keplr instance and enable the secret network
//   const setupKeplr = async (chainId: string) => {
//     // Define sleep
//     const sleep = (ms: number) => new Promise(accept => setTimeout(accept, ms))

//     // Wait for Keplr to be injected to the page
//     while (!window.keplr && !window.getOfflineSigner && !window.getEnigmaUtils) {
//       await sleep(10)
//     }

//     await window.keplr.experimentalSuggestChain(network)

//     // Enable Keplr.
//     // This pops-up a window for the user to allow keplr access to the webpage.
//     await window.keplr.enable(chainId)
//     setKeplkEnabled(true)
//   }

//   // Setup SecrtJS with Keplr's OfflineSigner
//   const setupClient = async () => {
//     // This pops-up a window for the user to sign on each tx we sent
//     const keplrOfflineSigner = window.getOfflineSigner(network.chainId)
//     const accounts = await keplrOfflineSigner.getAccounts()

//     const client = new SigningCosmWasmClient(
//       'https://bootstrap.secrettestnet.io', // holodeck - https://bootstrap.secrettestnet.io; mainnet - user your LCD/REST provider
//       accounts[0].address,
//       keplrOfflineSigner,
//       window.getEnigmaUtils(network.chainId),
//       {
//         // 300k - Max gas units we're willing to use for init
//         init: {
//           amount: [{ amount: '300000', denom: 'uscrt' }],
//           gas: '300000'
//         },
//         // 300k - Max gas units we're willing to use for exec
//         exec: {
//           amount: [{ amount: '980000', denom: 'uscrt' }],
//           gas: '980000'
//         }
//       }
//     )
//     if (isBrowser) window.scrtClient = client

//     const scrtAccount = await client?.getAccount(account?.address)
//     if (viewingKey === undefined) {
//       const vKey = await execute(contracts.bullpen.address, contracts.bullpen.messages.setViewingKey())
//       await execute(contracts.nft.address, contracts.nft.messages.setViewingKey())
//       try {
//         setViewingKey(vKey.viewing_key.key)
//       } catch (error) {}
//     }
//   }




//   return(
//     <div>
//       Kepl connecting
//     </div>
//   )
// }


// export default KeplrConnection
