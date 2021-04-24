/* eslint-disable jsx-a11y/interactive-supports-focus */
import { css } from '@emotion/core'
import React, { useContext } from 'react'
import { animated, useSpring } from 'react-spring'
import { colors } from '../styles/variables'
import { ChainId, KeplrContext } from './KeplrContext'

export interface WalletConnectProps {
  chaindId: ChainId
}

const WalletConnect: React.FC<WalletConnectProps> = ({ chaindId }) => {
  const { status, connect, resetViewingKey, account } = useContext(KeplrContext)
  const rx = status === 'working' ? 128 : 64
  const loaderOpacity = status === 'working' ? 0.75 : 0

  const animatedRx = useSpring({
    rx,
    config: {
      friction: 7
    }
  })

  let color = 'none'
  if (status === 'idle') {
    color = colors.cyan
  }

  if (status === 'failure') {
    color = colors.red
  }

  if (status === 'working') {
    color = colors.yellow
  }

  console.log(status)
  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events
    <div
      role="button"
      css={css`
        height: 80%;
        background: ${colors.gray.c900};
        border-radius: 15px;
      `}
      onClick={_ => {
        connect(chaindId)
      }}
    >
      <svg height="100%" viewBox="0 0 1900 256" version="1.1" xmlns="http://www.w3.org/2000/svg">
        <title>Kepler Connection</title>
        <defs>
          <linearGradient x1="7.27233149%" y1="5.60080287%" x2="93.8862913%" y2="92.9848946%" id="linearGradient-1">
            <stop stopColor="#3BA4CE" offset="0%" />
            <stop stopColor="#7868C8" offset="52.0850929%" />
            <stop stopColor="#B54BC1" offset="100%" />
          </linearGradient>
          <filter id="f1" x="0" y="0">
            <feGaussianBlur in="SourceGraphic" stdDeviation="2" />
          </filter>
        </defs>

        <g
          css={css`
            display: flex;
            flex-direction: row;
          `}
        >
          <g id="Page-1" stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
            <g id="k">
              <animated.rect id="Rectangle" fill="url(#linearGradient-1)" x="13" y="13" width="230" height="230" rx={animatedRx.rx} />

              <animated.rect
                x="20"
                y="20"
                width="216"
                height="216"
                // eslint-disable-next-line no-nested-ternary
                stroke={color}
                strokeWidth="12"
                rx={animatedRx.rx}
                filter="url(#f1)"
              />

              <polygon
                id="Path"
                fill="#FFFFFF"
                points="67 54.75 95.4172414 54.75 95.4172414 119.496204 152.675862 54.75 190 54.75 123.18131 127.75 190 200.75 152.675862 200.75 95.4172414 136.003796 95.4172414 200.75 67 200.75"
              />

              <svg width="200" height="200" x="28px" y="28px" viewBox="0 0 40 40" xmlns="http://www.w3.org/2000/svg">
                <defs>
                  <linearGradient x1="8.042%" y1="0%" x2="65.682%" y2="23.865%" id="a">
                    <stop stopColor="#fff" stopOpacity="0" offset="0%" />
                    <stop stopColor="#fff" stopOpacity=".631" offset="63.146%" />
                    <stop stopColor="#fff" offset="100%" />
                  </linearGradient>
                </defs>
                <g fill="none" fillRule="evenodd">
                  <g transform="translate(1 1)">
                    <path d="M36 18c0-9.94-8.06-18-18-18" id="Oval-2" stroke="url(#a)" strokeWidth="5" opacity={loaderOpacity}>
                      <animateTransform
                        attributeName="transform"
                        type="rotate"
                        from="0 18 18"
                        to="360 18 18"
                        dur="0.9s"
                        repeatCount="indefinite"
                      />
                    </path>
                    <circle fill="#fff" cx="36" cy="18" r="1" opacity={loaderOpacity}>
                      <animateTransform
                        attributeName="transform"
                        type="rotate"
                        from="0 18 18"
                        to="360 18 18"
                        dur="0.9s"
                        repeatCount="indefinite"
                      />
                    </circle>
                  </g>
                </g>
              </svg>
            </g>
          </g>
          <g>
            {status === 'undefined' && (
              <text
                x="300"
                y="155"
                id="text4549"
                css={css`
                  font-style: normal;
                  font-weight: normal;
                  font-size: 80px;
                  line-height: 1.25;
                  font-family: sans-serif;
                  letter-spacing: 0px;
                  word-spacing: 0px;
                  fill: ${colors.white};
                  fill-opacity: 1;
                  stroke: none;
                  stroke-width: 0.56458332;
                `}
              >
                Click to connect
              </text>
            )}

            {status === 'working' && (
              <text
                x="300"
                y="155"
                id="text4549"
                css={css`
                  font-style: normal;
                  font-weight: normal;
                  font-size: 80px;
                  line-height: 1.25;
                  font-family: sans-serif;
                  letter-spacing: 0px;
                  word-spacing: 0px;
                  fill: ${colors.white};
                  fill-opacity: 1;
                  stroke: none;
                  stroke-width: 0.56458332;
                `}
              >
                Working
              </text>
            )}

            {status === 'idle' && account?.address === undefined ? (
              <text
                x="300"
                y="155"
                id="text4549"
                css={css`
                  font-style: normal;
                  font-weight: normal;
                  font-size: 80px;
                  line-height: 1.25;
                  font-family: sans-serif;
                  letter-spacing: 0px;
                  word-spacing: 0px;
                  fill: ${colors.white};
                  fill-opacity: 1;
                  stroke: none;
                  stroke-width: 0.56458332;
                `}
              >
                Connected
              </text>
            ) : (
              <text
                x="300"
                y="155"
                id="text4549"
                css={css`
                  font-style: normal;
                  font-weight: normal;
                  font-size: 65px;
                  line-height: 1.25;
                  font-family: sans-serif;
                  letter-spacing: 0px;
                  word-spacing: 0px;
                  fill: ${colors.white};
                  fill-opacity: 1;
                  stroke: none;
                  stroke-width: 0.56458332;
                `}
              >
                {account?.address}
              </text>
            )}

            {status === 'failure' && (
              <text
                x="300"
                y="155"
                id="text4549"
                css={css`
                  font-style: normal;
                  font-weight: normal;
                  font-size: 80px;
                  line-height: 1.25;
                  font-family: sans-serif;
                  letter-spacing: 0px;
                  word-spacing: 0px;
                  fill: ${colors.white};
                  fill-opacity: 1;
                  stroke: none;
                  stroke-width: 0.56458332;
                `}
              >
                Error
              </text>
            )}
          </g>
        </g>
      </svg>
    </div>
  )
}

export default WalletConnect
