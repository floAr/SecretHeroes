/**
 * Implement Gatsby's Browser APIs in this file.
 *
 * See: https://www.gatsbyjs.org/docs/browser-apis/
 */
const React = require('react')
const { KeplrContextProvider } = require('./src/secret/KeplrContext')

// You can delete this file if you're not using it
export const wrapRootElement = ({ element }) => {
  return <KeplrContextProvider>{element}</KeplrContextProvider>
}
