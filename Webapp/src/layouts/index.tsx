import * as React from 'react'
import Helmet from 'react-helmet'
import 'modern-normalize'
import '../styles/normalize'

import { ToastContainer } from 'react-toastify'
import styled from '@emotion/styled'
import LayoutRoot from '../components/LayoutRoot'
import LayoutMain from '../components/LayoutMain'
import 'react-toastify/dist/ReactToastify.min.css'
import { colors, fontMain } from '../styles/variables'

// interface StaticQueryProps {
//   site: {
//     siteMetadata: {
//       title: string
//       description: string
//       keywords: string
//     }
//   }
// }

const StyledContainer = styled(ToastContainer)`
  .Toastify__toast-container {
  }
  .Toastify__toast {
    background: ${colors.gray.c200};
    color: ${colors.gray.c800};
    font-family: ${fontMain};
  }
  .Toastify__toast--error {
    background: ${colors.red};
    color: ${colors.gray.c200};
    font-family: ${fontMain};
  }
  .Toastify__toast--warning {
    background: ${colors.yellow};
    color: ${colors.gray.c800};
    font-family: ${fontMain};
  }
  .Toastify__toast--success {
    background: ${colors.cyan};
    color: ${colors.gray.c800};
    font-family: ${fontMain};
  }
  .Toastify__toast-body {
  }
  .Toastify__progress-bar {
  }
  .Toastify__close-button {
    color: ${colors.gray.c500};
  }
`

const IndexLayout: React.FC = ({ children }) => (
  <LayoutRoot>
    <Helmet title="Secret Heroes">
      <link rel="preconnect" href="https://fonts.gstatic.com" />
      <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;500&family=Zen+Dots&display=swap" rel="stylesheet" />
    </Helmet>

    <LayoutMain>{children}</LayoutMain>
    <StyledContainer autoClose={6000} pauseOnFocusLoss={false} position="bottom-right" />
  </LayoutRoot>
)

export default IndexLayout
