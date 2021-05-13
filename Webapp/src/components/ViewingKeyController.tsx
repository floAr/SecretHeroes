import { css } from '@emotion/core'
import React, { useContext, useEffect, useState } from 'react'
import { Contracts } from '../secret-heroes/contracts'
import { KeplrContext } from '../secret/KeplrContext'
import { viewingKeyContext } from '../secret/ViewingKeysContext'
import ViewingKeyTokenState from './ViewingKeyTokenState'

export interface ViewingKeyControllerProps {}

const ViewingKeyController: React.FC<ViewingKeyControllerProps> = () => {
  const isBrowser = typeof window !== 'undefined'
  const { getViewingKey, addViewingKey } = useContext(viewingKeyContext)

  const { connected, account, client } = useContext(KeplrContext)

  const [viewingKey, setViewingKey] = useState<string | undefined>(undefined)
  useEffect(() => {
    if (isBrowser && account?.address) setViewingKey(getViewingKey(account?.address))
  }, [account, getViewingKey])

  const newViewingKey = () => {
    if (isBrowser && account?.address) {
      const newKey = Contracts.helper.getEntropy()
      addViewingKey({ address: account?.address, key: newKey })
    }
  }

  return (
    <div
      css={css`
        display: flex;
        flex-direction: column;
      `}
    >
      <h3
        css={css`
          color: white;
        `}
      >
        Current Address
      </h3>
      <p
        css={css`
          color: white;
        `}
      >
        {account?.address}
      </p>
      <h3
        css={css`
          color: white;
        `}
      >
        Current ViewingKey
      </h3>
      {viewingKey != null ? (
        <div
          css={css`
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: baseline;
          `}
        >
          <p
            css={css`
              color: white;
            `}
          >
            {viewingKey}
          </p>
          <button
            type="button"
            onClick={_ => {
              newViewingKey()
            }}
          >
            Renew
          </button>
        </div>
      ) : (
        <button
          type="button"
          onClick={() => {
            newViewingKey()
          }}
        >
          Create ViewingKey
        </button>
      )}
      <h3
        css={css`
          color: white;
        `}
      >
        Contracts
      </h3>
      <ViewingKeyTokenState
        name="Token"
        address={Contracts.cards.address}
        query={async () => {
          console.log(client, account?.address, viewingKey)
          if (client != null && account?.address != null && viewingKey != null) {
            const list = await Contracts.cards.getAllTokens(client, account?.address, viewingKey)
            console.log('list', list)
            return list != null
          }
          return false
        }}
        set={async () => {
          if (client != null && account?.address != null && viewingKey != null) {
            return Contracts.cards.setViewingKey(client, viewingKey)
          }
        }}
      />
      <ViewingKeyTokenState
        name="Arena"
        address={Contracts.arena.address}
        query={async () => {
          if (client != null && account?.address != null && viewingKey != null) {
            const arena = await await Contracts.arena.fightStatusQuery(client, account?.address, viewingKey)
            return arena != null
          }
          return false
        }}
      />
    </div>
  )
}

export default ViewingKeyController
