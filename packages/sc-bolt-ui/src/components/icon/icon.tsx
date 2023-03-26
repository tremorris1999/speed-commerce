import { component$ } from '@builder.io/qwik'

declare global {
  // eslint-disable-next-line no-var
  var icons: { [key: string]: string }
}

if (!global.icons) {
  import('@mdi/js').then((mdi) => {
    global.icons = mdi as unknown as { [key: string]: string }
  })
}

const getIcon = (iconName: string) => {
  const icon = global.icons[iconName]
  return icon
}

export const Icon = component$((props: { iconName: string }) => {
  return (
    <>
      <svg style={'width:24px; height:24px'} viewBox="0 0 24 24">
        <path d={getIcon(props.iconName)} />
      </svg>
    </>
  )
})
