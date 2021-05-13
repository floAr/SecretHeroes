import { css } from '@emotion/core'
import React, { useContext, useEffect, useState } from 'react'
import { ViewingKeyRsp } from '../secret-heroes/contracts'

export type contractState = 'unknown' | 'ok' | 'error' | 'timeout'
export interface ViewingKeyTokenStateProps {
  viewingKey: string
  name: string
  address: string
  query: () => Promise<boolean>
  set: () => Promise<ViewingKeyRsp>
}
const ViewingKeyTokenState: React.FC<ViewingKeyTokenStateProps> = ({ name, address, query, set, viewingKey }) => {
  const [state, setState] = useState<contractState>('unknown')

  const getState = (s: contractState) => {
    switch (s) {
      case 'unknown':
        return <span>?</span>
      case 'ok':
        return <span>ok</span>
      case 'error':
        return <span>not set</span>
      case 'timeout':
        return <span>waiting</span>
      default:
        return <span />
    }
  }

  const check = async () => {
    const success = await query()
    setState(success ? 'ok' : 'error')
  }

  const add = async () => {
    const rsp = await set()
    if (rsp.viewing_key.key) {
      setState('ok')
      return
    }
    setState('timeout')
    setTimeout(() => {
      setState('error')
    }, 5000)
  }

  useEffect(() => {
    check()
  }, [query, viewingKey])

  return (
    <div
      css={css`
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: baseline;
      `}
    >
      <span>{name}</span>
      {getState(state)}
      {state === 'error' ? (
        <button
          type="button"
          onClick={_ => {
            add()
          }}
        >
          Set
        </button>
      ) : (
        <button
          type="button"
          onClick={_ => {
            check()
          }}
        >
          Check
        </button>
      )}
    </div>
  )
}

export default ViewingKeyTokenState
