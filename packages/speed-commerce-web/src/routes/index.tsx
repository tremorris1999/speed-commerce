import { component$ } from '@builder.io/qwik'
import type { DocumentHead } from '@builder.io/qwik-city'
import { Button } from 'qwik-bolt-ui'

export default component$(() => {
  return (
    <>
      <h1>Welcome to Qwik</h1>
      <div style={'background-color: #001f5f;'}>
        <Button variant="primary" theme="dark" size="xsmall">
          Goodbye Cruel World
          <span q:slot="append"> 👋</span>
        </Button>
      </div>
      <Button variant="secondary" theme="light" size="small">
        <span q:slot="prepend">👋 </span>
        Hello World
      </Button>
      <div style={'background-color: #001f5f;'}>
        <Button variant="primary" theme="dark" size="medium">
          Goodbye Cruel World
          <span q:slot="append"> 👋</span>
        </Button>
      </div>
      <Button variant="secondary" theme="light" size="large">
        <span q:slot="prepend">👋 </span>
        Hello World
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
