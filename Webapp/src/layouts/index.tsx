import * as React from 'react'
import Helmet from 'react-helmet'
import { StaticQuery, graphql } from 'gatsby'

import 'modern-normalize'
import '../styles/normalize'

import LayoutRoot from '../components/LayoutRoot'
import LayoutMain from '../components/LayoutMain'
import { KeplrContextProvider } from '../secret/KeplrContext'

// interface StaticQueryProps {
//   site: {
//     siteMetadata: {
//       title: string
//       description: string
//       keywords: string
//     }
//   }
// }

const IndexLayout: React.FC = ({ children }) => (
  <LayoutRoot>
    <Helmet title="Secret Heroes">
      <link rel="preconnect" href="https://fonts.gstatic.com" />
      <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;500&family=Zen+Dots&display=swap" rel="stylesheet" />
    </Helmet>

    <LayoutMain>{children}</LayoutMain>
  </LayoutRoot>
)

export default IndexLayout
