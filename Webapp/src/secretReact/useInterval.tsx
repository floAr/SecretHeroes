import { useEffect, useRef } from 'react'

export function useInterval(callback: (() => Promise<void>) | undefined, delay: number) {
  const savedCallback = useRef<() => Promise<void>>()

  useEffect(() => {
    savedCallback.current = callback
  }, [callback])

  // eslint-disable-next-line consistent-return
  useEffect(() => {
    function tick() {
      if (savedCallback && savedCallback.current) savedCallback.current()
    }
    if (delay !== null) {
      const id = setInterval(tick, delay)
      return () => {
        clearInterval(id)
      }
    }
  }, [callback, delay])
}
