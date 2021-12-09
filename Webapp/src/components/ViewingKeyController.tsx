import { css } from '@emotion/core'
import React, { useContext, useEffect, useState } from 'react'
import { Contracts } from '../secret-heroes/contracts'
import { KeplrContext } from '../secret/KeplrContext'
import { viewingKeyContext } from '../secret/ViewingKeysContext'
import ViewingKeyTokenState from './ViewingKeyTokenState'

export interface ViewingKeyControllerProps {
  setValid: React.Dispatch<React.SetStateAction<boolean>>
}

const ViewingKeyController: React.FC<ViewingKeyControllerProps> = ({ setValid }) => {
  const isBrowser = typeof window !== 'undefined'
  const { getViewingKey, addViewingKey } = useContext(viewingKeyContext)

  const { connected, account, client } = useContext(KeplrContext)

  const [viewingKey, setViewingKey] = useState<string | undefined>(undefined)

  // migrate existing viewingkeys
  useEffect(() => {
    const existing = localStorage.getItem('viewingKey')
    if (existing != null && existing.length > 0 && account?.address) {
      localStorage.removeItem('viewingKey')
      addViewingKey({ address: account?.address, key: existing })
      console.log('migrated viewingkey', existing)
    }
  }, [account])

  useEffect(() => {
    if (isBrowser && account?.address) setViewingKey(getViewingKey(account?.address))
  }, [account, getViewingKey])

  const newViewingKey = () => {
    if (isBrowser && account?.address) {
      const newKey = Contracts.helper.getEntropy()
      setViewingKey(newKey)
      addViewingKey({ address: account?.address, key: newKey })
    }
  }

  const [tokenValid, setTokenValid] = useState<boolean>(false)
  const [arenaValid, setArenaValid] = useState<boolean>(false)

  useEffect(() => {
    if (setValid) setValid(tokenValid && arenaValid)
  }, [tokenValid, arenaValid, viewingKey])

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
        {account?.address ? account?.address : 'Please connect to Keplr'}
      </p>
      {connected && (
        <h3
          css={css`
            color: white;
          `}
        >
          Current ViewingKey
        </h3>
      )}
      {viewingKey != null && connected && (
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
      )}
      {viewingKey == null && connected && (
        <button
          type="button"
          onClick={() => {
            newViewingKey()
          }}
        >
          Create ViewingKey
        </button>
      )}

      {viewingKey && (
        <>
          <h3
            css={css`
              color: white;
            `}
          >
            Contracts
          </h3>
          <ViewingKeyTokenState
            viewingKey={viewingKey}
            name="Token"
            address={Contracts.cards.address}
            query={async () => {
              if (client != null && account?.address != null && viewingKey != null) {
                const list = await Contracts.cards.getAllTokens(client, account?.address, viewingKey)
                return list != null
              }
              return false
            }}
            set={async () => {
              if (client != null && account?.address != null && viewingKey != null) {
                return Contracts.cards.setViewingKey(client, viewingKey)
              }
              return undefined
            }}
            setValid={setTokenValid}
          />
          <ViewingKeyTokenState
            viewingKey={viewingKey}
            name="Arena"
            address={Contracts.arena.address}
            query={async () => {
              if (client != null && account?.address != null && viewingKey != null) {
                const arena = await await Contracts.arena.fightStatusQuery(client, account?.address, viewingKey)
                return arena != null
              }
              return false
            }}
            set={async () => {
              if (client != null && account?.address != null && viewingKey != null) {
                return Contracts.arena.setViewingKey(client, viewingKey)
              }
              return undefined
            }}
            setValid={setArenaValid}
          />
        </>
      )}
    </div>
  )
}

export default ViewingKeyController
