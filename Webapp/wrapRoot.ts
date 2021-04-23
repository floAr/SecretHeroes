import React from 'react'
import { KeplrContextProvider } from './src/secret/KeplrContext'

export const wrapRootElement = ({ element }) => {
  return (<KeplrContextProvider>{ element } < /KeplrContextProvider>)
}
