import { component$, Slot } from '@builder.io/qwik'
import { loader$ } from '@builder.io/qwik-city'

import Header from '../components/header/header'

export const useServerTimeLoader = loader$(() => {
  return {
    date: new Date().toISOString()
  }
})

export default component$(() => {
  return (
    <>
      <main>
        <Header />
        <section>
          <Slot />
        </section>
      </main>
    </>
  )
})
