'use strict'

module.exports = {
  siteMetadata: {
    title: 'gatsby-starter-typescript-plus',
    description: 'A starter kit for TypeScript-based Gatsby projects with sensible defaults.',
    keywords: 'gatsbyjs, gatsby, javascript, sample, something',
    siteUrl: 'https://secrethero.es',
    author: {
      name: 'Florian Uhde',
      url: 'https://twitter.com/florianuhde'
    },
    "og:title": 'SecretHeroes',
    "og:type": 'website',
    "og:image": 'https://secrethero.es/image/og.png',
    "og:url": 'https://secrethero.es/',
    "og:description": 'Collect. Battle. Dominate. An NFT-based game secured by Secret Network.'
  },
  flags: { PRESERVE_WEBPACK_CACHE: true },
  plugins: [
    'gatsby-plugin-emotion',
    'gatsby-plugin-typescript',
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/src/images/`
      }
    },
    'gatsby-transformer-sharp',
    'gatsby-plugin-react-helmet'
  ]
}
