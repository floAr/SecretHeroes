import * as React from 'react'
import Helmet from 'react-helmet'
import 'modern-normalize'
import '../styles/normalize'

import { ToastContainer } from 'react-toastify'
import { css } from '@emotion/core'
import styled from '@emotion/styled'
import LayoutRoot from '../components/LayoutRoot'
import LayoutMain from '../components/LayoutMain'
import 'react-toastify/dist/ReactToastify.min.css'
import { colors, fontMain } from '../styles/variables'
import Header from '../components/Header'
import { Link } from 'gatsby'

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
      <link
        href="https://fonts.googleapis.com/css2?family=Inter:wght@200;400;700&family=Roboto:ital,wght@0,400;0,700;1,100&display=swap"
        rel="stylesheet"
      />
      <meta property="og:title" content="Secret Heroes - NFT Gaming" />
      <meta property="og:type" content="website" />
      <meta property="og:image" content="https://secrethero.es/images/og.png" />
      <meta property="og:url" content="https://secrethero.es/" />
      <meta property="og:description" content="Collect. Battle. Dominate. An NFT-based game secured by Secret Network." />
    </Helmet>
    <Header />
    <LayoutMain>{children}</LayoutMain>
    <StyledContainer autoClose={6000} pauseOnFocusLoss={false} position="bottom-right" />
    <section className="footer" css={css`
      background: #000;
      padding-top: 32px;
      padding-bottom: 32px;
      display: grid;
      text-align: center;
      gap: 16px;
      `}>
        <div className="social" css={css`
          `}>
          <Link to="https://twitter.com/secretheroesnft" target="new" css={css`
          width: 24px;
          height: 24px;
          display: inline-block;
          background: url('../images/twitter.png');
          `}></Link>
          <Link to="https://discord.gg/JpTnNRVzpw" target="new" css={css`
          width: 24px;
          height: 24px;
          display: inline-block;
          margin-left: 16px;
          background: url('../images/discord.png');
          `}></Link>
        </div>
        <p>© Secret Heroes</p>
      </section>
  </LayoutRoot>
)

export default IndexLayout
