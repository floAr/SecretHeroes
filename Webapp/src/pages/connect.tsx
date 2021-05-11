import * as React from 'react'

import { css, keyframes } from '@emotion/core'
import { useContext, useEffect } from 'react'
import { navigate } from 'gatsby'
import IndexLayout from '../layouts'
import { KeplrContext } from '../secret/KeplrContext'

const PleaseConnectPage = () => {
  const { connected } = useContext(KeplrContext)
  const bounce = keyframes`
  from, 20%, 53%, 80%, to {
    transform: translate3d(0,0,0);
  }

  40%, 43% {
    transform: translate3d( -20px,0, 0);
  }

  70% {
    transform: translate3d( -10px,0, 0);
  }

  90% {
    transform: translate3d(-4px,0,0);
  }
`

  useEffect(() => {
    if (connected) {
      navigate('/game')
    }
  }, [connected])
  return (
    <IndexLayout>
      <div
        css={css`
          display: flex;
          justify-content: flex-end;
        `}
      >
        <div
          css={css`
            display: flex;
            flex-direction: column;
            margin-right: 32px;
            margin-top: 100px;
            align-items: flex-start;
          `}
        >
          <div
            css={css`
              transform: rotate(-90deg) scale(0.6);
              width: 80px;
              height: 60px;
              margin-left: 30px;
            `}
          >
            <div
              id="icon"
              css={css`
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                width: 80px;
                height: 60px;
                cursor: pointer;
                padding: 10px;
              `}
            >
              <div
                id="arrow"
                css={css`
                  position: absolute;
                  top: 25px;
                  width: 90%;
                  height: 10px;
                  background-color: #fff;
                  box-shadow: 0 3px 5px rgba(0, 0, 0, 0.2);
                  animation: ${bounce} 1s ease infinite;

                  &:after,
                  &:before {
                    content: '';
                    position: absolute;
                    width: 60%;
                    height: 10px;
                    right: -8px;
                    background-color: #fff;
                  }

                  &:after {
                    top: -12px;
                    transform: rotate(45deg);
                  }

                  &:before {
                    top: 12px;
                    box-shadow: 0 3px 5px rgba(0, 0, 0, 0.2);
                    transform: rotate(-45deg);
                  }
                `}
              />
            </div>
          </div>
          <p
            css={css`
              font-size: 1.5rem;
              font-weight: 500;
            `}
          >
            Please connect to Secret Network
          </p>
        </div>
      </div>
    </IndexLayout>
  )
}

export default PleaseConnectPage
