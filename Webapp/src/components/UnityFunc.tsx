import css from '@emotion/css'
import { Link } from 'gatsby'
import React, { useEffect, useState } from 'react'
import Modal from 'react-modal'

const UnityFunc: React.FC = () => {
  const [howToPlayOpen, setHowToPlayOpen] = useState<boolean>(false)
  const [aboutOpen, setAboutOpen] = useState<boolean>(false)

  const isBrowser = typeof window !== 'undefined'
  useEffect(() => {
    if (!isBrowser) return
    const buildUrl = '/Build'
    const loaderUrl = `${buildUrl}/_build.loader.js`
    const config = {
      dataUrl: `${buildUrl}/_build.data`,
      frameworkUrl: `${buildUrl}/_build.framework.js`,
      codeUrl: `${buildUrl}/_build.wasm`,
      streamingAssetsUrl: 'StreamingAssets',
      companyName: 'DefaultCompany',
      productName: 'Client',
      productVersion: '1.0'
    }

    const container = document.querySelector('#unity-container')
    const canvas = document.querySelector('#unity-canvas')
    const loadingBar = document.querySelector('#unity-loading-bar')
    const progressBarFull = document.querySelector('#unity-progress-bar-full')
    const fullscreenButton = document.querySelector('#unity-fullscreen-button')
    const mobileWarning = document.querySelector('#unity-mobile-warning')

    if (/iPhone|iPad|iPod|Android/i.test(navigator.userAgent)) {
      container.className = 'unity-mobile'
      config.devicePixelRatio = 1
      mobileWarning.style.display = 'block'
      setTimeout(() => {
        mobileWarning.style.display = 'none'
      }, 5000)
    } else {
      canvas.style.width = '100%'
    }
    loadingBar.style.display = 'block'

    const script = document.createElement('script')
    script.src = loaderUrl
    script.onload = () => {
      createUnityInstance(canvas, config, progress => {
        progressBarFull.style.width = `${100 * progress}%`
      })
        .then(unityInstance => {
          window.unityInstance = unityInstance

          loadingBar.style.display = 'none'
          // fullscreenButton.onclick = () => {
          //   unityInstance.SetFullscreen(1)
          // }
          window.registerUnityInstance(unityInstance)
        })
        .catch(message => {
          alert(message)
        })
    }
    document.body.appendChild(script)
  }, [])

  return (
    <div
      id="unity-container"
      className="unity-desktop"
      css={css`
        width: 100%;
      `}
    >
      <canvas id="unity-canvas" />
      <div id="unity-loading-bar">
        <div id="unity-logo" />
        <div id="unity-progress-bar-empty">
          <div id="unity-progress-bar-full" />
        </div>
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: row;
        `}
      >
        <button
          onClick={e => {
            setHowToPlayOpen(true)
          }}
        >
          How to play
        </button>
        <button
          onClick={e => {
            setAboutOpen(true)
          }}
        >
          About
        </button>
      </div>
      <Modal isOpen={howToPlayOpen} contentLabel="How To Play" ariaHideApp={false}>
        <button
          onClick={e => {
            setHowToPlayOpen(false)
          }}
        >
          X
        </button>
        <h3>How to play</h3>
        <ul>
          <li>You can mint packs of 3 cards in the market</li>
          <li>Each pack contains one hero card that can be unwrapped</li>
          <li>Heroes join your rooster and can be sent to battle in the bar</li>
          <li>When three players have sent their hero the battle begins</li>
          <li>The smart contract picks a random skill and the card with the highest skill value wins</li>
          <li>The owner of the winning card receives all participating Heroes</li>
        </ul>
      </Modal>
      <Modal isOpen={aboutOpen} contentLabel="How To Play" ariaHideApp={false}>
        <button
          onClick={e => {
            setAboutOpen(false)
          }}
        >
          X
        </button>
        <h3>The Game</h3>
        Secret Heroes is a card game based on{' '}
        <Link to="https://scrt.network/blog/secret-grant-secret-nft-reference-implementation">secretNFTs, the SNIP-721</Link> standard on
        Secret Network. Leveraging privacy properties of
        <Link to="https://build.scrt.network/">Secret Network</Link>,{' '}
        <Link to="https://github.com/baedrik/snip721-reference-impl">SNIP-721</Link> enables private ownership and private metadata in NFTs.
        Players can buy NFT cards (heroes) with four different skills. Players can “battle” their cards against 2 other players. The game
        randomly determines a skill to evaluate and determines a winner based on which card has the highest value for that skill. The winner
        receives all the cards used in the battle. No player has any information about other players’ cards at any point excpept the winning
        card.
        <h4>Found and issues?</h4>
        We are trying to make Secret Heroes into something as close as possible to a real game. So please feel free to report any issues you
        encouter <Link to="https://github.com/floAr/NFTHack-SecretHeroes/issues">here.</Link>
      </Modal>
    </div>
  )
}

export default React.memo(UnityFunc)
