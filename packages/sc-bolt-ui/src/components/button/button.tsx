import {
  QwikMouseEvent,
  Slot,
  component$,
  useStylesScoped$
} from '@builder.io/qwik'
import styles from './button.scss?inline'
export interface ButtonProps {
  onClick?: (event: QwikMouseEvent<HTMLElement, MouseEvent>) => void
  theme?: 'light' | 'dark'
  variant?: 'primary' | 'secondary'
  text?: string
  size?: 'xsmall' | 'small' | 'medium' | 'large'
  rounded?: boolean
}

const determineClass = (props: ButtonProps): string => {
  const theme = props.theme || 'light'
  const variant = props.variant || 'primary'
  const size = props.size || 'medium'
  const rounded = props.rounded ? 'rounded' : 'boxed'
  return `${theme} ${variant} ${size} ${rounded}`
}

const mainContent = (props: ButtonProps) => {
  if (props.text) {
    return props.text
  } else {
    return <Slot />
  }
}

export const Button = component$((props: ButtonProps) => {
  useStylesScoped$(styles)
  return (
    <>
      <button
        type="button"
        class={determineClass(props)}
        preventdefault:click
        onClick$={props.onClick}
      >
        <Slot name="prepend" />
        <div id="main-content">{mainContent(props)}</div>
        <Slot name="append" />
      </button>
    </>
  )
})
