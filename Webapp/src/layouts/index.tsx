import * as React from 'react'
import Helmet from 'react-helmet'
import { StaticQuery, graphql } from 'gatsby'

import 'modern-normalize'
import '../styles/normalize'

import LayoutRoot from '../components/LayoutRoot'
import LayoutMain from '../components/LayoutMain'

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
    <Helmet title="Secret Heroes" />
    <LayoutMain>{children}</LayoutMain>
  </LayoutRoot>
)

export default IndexLayout
