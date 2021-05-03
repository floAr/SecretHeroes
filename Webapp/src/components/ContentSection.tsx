import { css } from '@emotion/core'
import React from 'react'
import Image from './Image'

export interface ContentSectionProps {
  backgroundImg: string
  mainImg: string
  content: string
  cta: string
  title: string
}

const ContentSection: React.FC<ContentSectionProps> = ({ cta, title, content, mainImg, backgroundImg }) => {
  return (
    <div
      className="section-background"
      css={css`
        width: 100%;
        display: flex;
        flex-direction: row;
        align-items: center;
        flex-wrap: wrap;
        justify-content: center;

        background: url(${backgroundImg});
        background-position: 50% 50%;
        background-size: cover;
        border-top: solid;

        padding-top: 5vh;
        padding-bottom: 5vh;
      `}
    >
      <div
        className="content-image"
        css={css`
          width: 45vw;
          display: flex;
          justify-content: center;
        `}
      >
        <Image fileName={mainImg} alt="" style={{ width: '60vh' }} />
      </div>
      <div
        css={css`
          margin: 1vw;
          padding-right: 5vw;
          padding-left: 5vw;
          width: 45vw;
          min-width: 600px;
        `}
      >
        <div
          className="content-text"
          css={css`
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: left;
          `}
        >
          <div
            css={css`
              font-family: Inter;
              font-style: normal;
              font-weight: bold;
              font-size: 16px;
              line-height: 19px;

              color: #fc2748;
            `}
          >
            {cta}
          </div>
          <div
            css={css`
              font-family: Inter;
              font-style: normal;
              font-weight: bold;
              font-size: 30px;
              line-height: 36px;

              color: #ffffff;
            `}
          >
            {title}
          </div>
          <div
            css={css`
              font-family: Roboto Mono;
              font-style: normal;
              font-weight: normal;
              font-size: 18px;
              line-height: 150%;
            `}
          >
            {content}
          </div>
        </div>
      </div>
    </div>
  )
}

export default ContentSection
