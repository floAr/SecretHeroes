import React, { useEffect, useState } from 'react'

interface TimeFrame {
  hours: string
  minutes: string
  seconds: string
}

const Launch: React.FC = () => {
  const zeroPad = (num: number, places: number) => String(num).padStart(places, '0')
  const calculateTimeLeft = () => {
    const difference = +new Date(`05/06/2021`) - +new Date()
    let timeLeft: TimeFrame = { hours: '', minutes: '', seconds: '' }

    if (difference > 0) {
      timeLeft = {
        hours: zeroPad(Math.floor((difference / (1000 * 60 * 60)) % 24) + Math.floor(difference / (1000 * 60 * 60 * 24)) * 24, 2),
        minutes: zeroPad(Math.floor((difference / 1000 / 60) % 60), 2),
        seconds: zeroPad(Math.floor((difference / 1000) % 60), 2)
      }
    }

    return timeLeft
  }

  const [timeLeft, setTimeLeft] = useState(calculateTimeLeft())

  useEffect(() => {
    const timer = setTimeout(() => {
      setTimeLeft(calculateTimeLeft())
    }, 1000)
    // Clear timeout if the component is unmounted
    return () => clearTimeout(timer)
  })

  const timerComponents: React.ReactNodeArray = []

  Object.keys(timeLeft).forEach(interval => {
    if (!timeLeft[interval as keyof TimeFrame]) {
      return
    }

    timerComponents.push(
      <span>
        {timeLeft[interval as keyof TimeFrame]}
        {(interval as keyof TimeFrame) !== 'seconds' ? ':' : ''}
      </span>
    )
  })

  return <div>{timerComponents.length ? timerComponents : <span>Time's up!</span>}</div>
}

export default Launch
