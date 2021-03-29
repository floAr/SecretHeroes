const mintMsg = (name1: string, name2: string, name3: string) => {
  return { mint: { names: [name1, name2, name3] } }
}
const minter = {
  address: 'secret1l7nmj0k9ny4dcq0w28t7kqwdqs8spxqar8ge3p',
  messages: {
    mint: mintMsg
  }
}

const setViewingKey = () => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { set_viewing_key: { key: '1' } }
}

const makeid = (length: number) => {
  let result = ''
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  const charactersLength = characters.length
  for (let i = 0; i < length; i += 1) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength))
  }
  return result
}

const getEntropy = () => {
  return btoa(makeid(15))
}

const fightStatus = (address: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { bullpen: { address, viewing_key: '1' } }
}

const fightHistory = (address: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { battle_history: { address, viewing_key: '1' } }
}

const returnFigher = () => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { chicken_out: {} }
}

const bullpen = {
  address: 'secret1efx2eqsycywclz37j5m9gz8x3lwpf5m3vqs75p',
  messages: {
    returnFigher,
    setViewingKey
  },
  queries: {
    fightStatus,
    fightHistory
  }
}



const nftDossier = (name: string, address: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { nft_dossier: { token_id: name, viewer: { address, viewing_key: '1' } } }
}

const privateMetadata = (name: string, address: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { private_metadata: { token_id: name, viewer: { address, viewing_key: '1' } } }
}

const getAllTokens = (address: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { tokens: { owner: address, viewing_key: '1' } }
}

const sendNft = (name: string) => {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return { send_nft: { contract: bullpen.address, token_id: name, msg: getEntropy() } }
}

const nft = {
  address: 'secret188fumr0mywykseuktfsjwpgz3us5zc48qk6n22',
  messages: {
    setViewingKey,
    sendNft
  },
  queries: {
    nftDossier,
    privateMetadata,
    getAllTokens
  }
}

export default { minter, nft, bullpen, getEntropy }

//  +++++++++++++++++++ MINTER +++++++++++++++++++++++++++++++++++
// window.scrtExec(window.scrtContracts.minter.address, window.scrtContracts.minter.messages.mint("test","fluhde","floAr"))
// {
//   "status": {
//       "status": "Success"
//   }
// }
//  +++++++++++++++++++ NFT ++++++++++++++++++++++++++++++++++++++
// window.scrtClient.queryContractSmart(window.scrtContracts.nft.address, window.scrtContracts.nft.queries.getAllTokens(window.scrtAccount.address))
// {
//   "token_list": {
//       "tokens": [
//           "floAr",
//           "fluhde",
//           "test"
//       ]
//   }
// }
// window.scrtClient.queryContractSmart(window.scrtContracts.nft.address, window.scrtContracts.nft.queries.privateMetadata("test",window.scrtAccount.address))
// {
//   "private_metadata": {
//       "name": "test",
//       "description": null,
//       "image": "[17,2,14,7]"
//   }
// }
// window.scrtClient.queryContractSmart(window.scrtContracts.nft.address, window.scrtContracts.nft.queries.nftDossier("test",window.scrtAccount.address))
// {
//   "nft_dossier": {
//       "owner": "secret17ra39gfxwhwtu4j5qhx8rfqmwxdxjeuxd8kw6c",
//       "public_metadata": {
//           "name": "test",
//           "description": null,
//           "image": null
//       },
//       "private_metadata": {
//           "name": "test",
//           "description": null,
//           "image": "[17,2,14,7]"
//       },
//       "display_private_metadata_error": null,
//       "owner_is_public": false,
//       "public_ownership_expiration": null,
//       "private_metadata_is_public": false,
//       "private_metadata_is_public_expiration": null,
//       "token_approvals": [],
//       "inventory_approvals": []
//   }
// }
// window.scrtExec(window.scrtContracts.nft.address, window.scrtContracts.nft.messages.setViewingKey())
// window.scrtExec(window.scrtContracts.nft.address, window.scrtContracts.nft.messages.sendNft("test"))

//  +++++++++++++++++++ BULLPED ++++++++++++++++++++++++++++++++++

// Get fighert status
// window.scrtClient.queryContractSmart(window.scrtContracts.bullpen.address, window.scrtContracts.bullpen.queries.fightStatus(window.scrtAccount.address))
// Before sending
// {
//   "bullpen": {
//       "heroes_waiting": 0,
//       "your_hero": null
//   }
// }
// AFTER SENDING
// {
//   "bullpen": {
//       "heroes_waiting": 1,
//       "your_hero": {
//           "token_id": "test",
//           "name": "test",
//           "skills": [
//               17,
//               2,
//               14,
//               7
//           ]
//       }
//   }
// }
// window.scrtExec(window.scrtContracts.bullpen.address, window.scrtContracts.bullpen.messages.returnFigher())

// window.scrtClient.queryContractSmart(window.scrtContracts.bullpen.address, window.scrtContracts.bullpen.queries.fightHistory(window.scrtAccount.address))
// NO FIGHTS
// {
//   "battle_history": {
//       "history": []
//   }
// }
