import { component$ } from '@builder.io/qwik'
import type { DocumentHead } from '@builder.io/qwik-city'
import { Icon, Button } from '@sc/bolt-ui'

export default component$(() => {
  return (
    <>
      <h1 style={'color: black;'}>Welcome to Qwik</h1>
      <Button theme="dark">
        <Icon q:slot="prepend" /> AAAA
      </Button>
    </>
  )
})

export const head: DocumentHead = {
  title: 'Welcome to Qwik',
  meta: [
    {
      name: 'description',
      content: 'Qwik site description'
    }
  ]
}
