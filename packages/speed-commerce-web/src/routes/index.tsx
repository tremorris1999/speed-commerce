import { component$ } from '@builder.io/qwik'
import type { DocumentHead } from '@builder.io/qwik-city'
import { Button } from 'qwik-bolt-ui'

export default component$(() => {
  return (
    <>
      <h1>Welcome to Qwik</h1>
      <Button text="Test Button" variant="primary" theme="dark" />
      <Button variant="primary" theme="dark" />
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
