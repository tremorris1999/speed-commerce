import { component$, useStylesScoped$ } from '@builder.io/qwik'
import styles from './button.scss?inline'
export interface ButtonProps {
  theme?: 'light' | 'dark'
  variant?: 'primary' | 'secondary'
  text?: string
}

const determineClass = (props: ButtonProps): string => {
  const theme = props.theme || 'light'
  const variant = props.variant || 'primary'
  return `${theme} ${variant}`
}

export const Button = component$((props: ButtonProps) => {
  useStylesScoped$(styles)
  return (
    <>
      <button class={determineClass(props)}>{props.text ?? 'Button'}</button>
    </>
  )
})
