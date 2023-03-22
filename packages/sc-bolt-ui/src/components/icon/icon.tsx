import { component$ } from '@builder.io/qwik'
import { mdiAlertMinusOutline } from '@mdi/js'

export const Icon = component$(() => {
  return (
    <>
      <svg style={'width:24px; height:24px'} viewBox="0 0 24 24">
        <path d={mdiAlertMinusOutline} style={'fill: black;'} />
      </svg>
    </>
  )
})
