import { component$ } from '@builder.io/qwik'

export const Icon = component$((props: { iconName: string }) => {
  return (
    <>
      <svg style={'width:24px; height:24px'} viewBox="0 0 24 24">
        <path d={global.icons[props.iconName]} />
      </svg>
    </>
  )
})
