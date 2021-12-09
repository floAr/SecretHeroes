/**
 * Implement Gatsby's Browser APIs in this file.
 *
 * See: https://www.gatsbyjs.org/docs/browser-apis/
 */
const React = require('react')
const { KeplrContextProvider } = require('./src/secret/KeplrContext')
const { ViewingKeyContextProvider } = require('./src/secret/ViewingKeysContext')

// You can delete this file if you're not using it
export const wrapRootElement = ({ element }) => {
  return (
    <ViewingKeyContextProvider>
      <KeplrContextProvider>{element} </KeplrContextProvider>
    </ViewingKeyContextProvider>
  )
}
