import React from 'react'
import { graphql, useStaticQuery } from 'gatsby'
import Img from 'gatsby-image'
import { css } from '@emotion/core'

export interface ImageProps {
  fileName: string
  alt: string
  style: any
}
const Image: React.FC<ImageProps> = ({ fileName, alt, style }) => {
  const { allImageSharp } = useStaticQuery(graphql`
    query {
      allImageSharp {
        nodes {
          fluid(maxWidth: 1600) {
            originalName
            ...GatsbyImageSharpFluid_withWebp
          }
        }
      }
    }
  `)

  const { fluid } = allImageSharp.nodes.find(n => n.fluid.originalName === fileName)

  return (
    <div
      css={css`
        display: flex;
        place-items: center;
        height: 100%;
      `}
    >
      <figure
        css={css`
          height: 100%;
          width: 100%;
          max-width: 100%;
        `}
      >
        <Img fluid={fluid} alt={alt} style={style} />
      </figure>
    </div>
  )
}

export default Image
