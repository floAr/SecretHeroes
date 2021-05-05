import { css } from '@emotion/core'
import React from 'react'
import Image from './Image'

export interface ContentSectionProps {
  backgroundImg: string
  mainImg: string
  content: string
  secondaryTitle: string
  secondaryContent: string
  secondaryImage: string
  visible: string
  cta: string
  title: string
}


const ContentSection: React.FC<ContentSectionProps> = ({ cta, title, content, secondaryTitle, secondaryContent, secondaryImage, visible, mainImg, backgroundImg }) => {
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

        padding-top: 15vh;
        padding-bottom: 15vh;
      `}
    >
      <div
        className="content-image"
        css={css`
          width: 45vw;
          display: flex;
          justify-content: center;
          @media only screen and (max-width: 768px) {
            text-align: center;
            img {
              max-width: 90vw;
              justify-items: center;
              position: absolute;
              margin-left: auto;
              margin-right: auto;
              left: 0;
              right: 0;
              text-align: center;
            }
          }
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
          @media only screen and (max-width: 768px) {
            min-width: 100%;
            max-width: 100%;
          }
        `}
      >
        <div
          className="content-text"
          css={css`
            display: grid;
            justify-content: center;
            align-items: left;
            gap:16px;
          `}
        >
          <div css={css`
          display: grid;
          gap: 8px;
          `}>
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
          </div>
          <div
            css={css`
            font-family: Inter;
            font-style: normal;
            font-weight: normal;
            font-size: 18px;
            line-height: 150%;
            margin-bottom: 20px;
            color: #9AA5B1;
            `}
          >
            {content}
          </div>
          <div
            css={css`
            grid-auto-flow: column;
            align-items: flex-start;
            padding: 0px;
            gap: 16px;
            display:none;
            display:${visible};

            `}
          >
            <div className={`${secondaryImage}`} css={css`
            background: #FC2748;
            background-image: url(${secondaryImage});
            background-repeat: no-repeat;
            background-position: center center;
            width: 48px;
            height: 48px;
            border-radius: 6px;
            `}></div>
            <div>
              <h6 css={css`
              font-family: Inter;
              font-style: normal;
              font-weight: 500;
              font-size: 18px;
              line-height: 24px;
              /* identical to box height, or 133% */
              margin-top: 0;

              color: #FFFFFF;
              `}>{secondaryTitle}</h6>
              <p css={css`
              font-family: Inter;
              font-style: normal;
              font-weight: normal;
              font-size: 16px;
              line-height: 24px;
              /* or 150% */


              /* Neutral/#400 */

              color: #9AA5B1;`}>{secondaryContent}</p></div>
          </div>
        </div>
      </div>
    </div >
  )
}

export default ContentSection
