declare global {
  // eslint-disable-next-line no-var
  var icons: { [key: string]: string }
}

if (!global.icons) {
  import('@mdi/js').then((mdi) => {
    global.icons = mdi as Omit<typeof mdi, 'default'>
  })
}

export { Button } from './components/button/button'
export { Icon } from './components/icon/icon'
