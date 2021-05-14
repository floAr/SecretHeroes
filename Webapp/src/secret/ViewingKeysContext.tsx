import React, { createContext, useCallback, useEffect, useMemo, useState } from 'react'

export type viewingKeyPair = {
  key: string
  address: string
}

const storageKey = 'viewingKeys'

export interface ViewingKeyContext {
  getViewingKey: (address: string) => string | undefined
  addViewingKey: (keyPair: viewingKeyPair) => void
}

export const viewingKeyContext = createContext<ViewingKeyContext>({
  getViewingKey: (_: string) => undefined,
  addViewingKey: (_: viewingKeyPair) => {
    // do nothing
  }
})

const ViewingKeyContextProvider: React.FC = ({ children }) => {
  const isBrowser = typeof window !== 'undefined'
  const [viewingKey, setViewingKey] = useState<viewingKeyPair[]>(isBrowser ? JSON.parse(localStorage.getItem(storageKey) ?? '[]') : [])

  useEffect(() => {
    localStorage.setItem(storageKey, JSON.stringify(viewingKey))
  }, [viewingKey])

  const getViewingKey = useCallback(
    (address: string) => {
      const item = viewingKey.find(vk => vk.address === address)
      if (item) return item.key
      return undefined
    },
    [viewingKey]
  )

  const addNewViewingKey = useCallback(
    (vkp: viewingKeyPair) => {
      const other = viewingKey.filter(vk => vk.address !== vkp.address)
      setViewingKey([...other, vkp])
    },
    [viewingKey]
  )

  const value = useMemo(() => {
    return { getViewingKey, addViewingKey: addNewViewingKey }
  }, [getViewingKey, addNewViewingKey])

  return <viewingKeyContext.Provider value={value}>{children}</viewingKeyContext.Provider>
}
export { ViewingKeyContextProvider, viewingKeyContext as ViewingKeyContext }
