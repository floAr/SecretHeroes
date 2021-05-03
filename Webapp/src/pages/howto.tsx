import * as React from 'react'
import { Link } from 'gatsby'

import Container from '../components/Container'
import IndexLayout from '../layouts'

const HowToPage = () => (
  <IndexLayout>
    <Container>
      <h1>How To Play</h1>
      <p>Work in progress</p>
      <Link to="/">Go back.</Link>
    </Container>
  </IndexLayout>
)

export default HowToPage
