# Front

This document should outline our intended front-end architecure.

## Server

We will use [nginx](https://nginx.org/en/) to host our front end in a docker container. We will configure routing to enable SPA navigation rather than serving different html files directly.

## Runtime

The intended runtime we will use here is [bun](https://bun.sh). Bun boasts incredible performance gains over Node, and will be used throughout development for running scripts and managing packages in automated and manualm scenarios. At present, bun is planned to have (but lacks) support for treeshaking and minifying. We will explore these options as they become available.

## Building

We will be utilizing [Nx](https://nx.dev) for caching build artifacts for quicker reuse. This will likely be more useful in CI/CD situations. Additionally, we will be using [Vite](https://vitejs.dev) for bundling, minifying, and treeshaking.

## UI

Our UI will be based on the bleeding-edge [Qwik](https://qwik.builder.io) framework. This will enable us to write our own application-specific component library and use it across our app. We will be using [Dart Sass](https://sass-lang.com/dart-sass) for styling according to the style guidelines provided by [Bolt](https://boltdesignsystem.com/).

## Testing 

For testing, we will use [Playwright](https://playwright.dev/) for E2E testing. We will benchmark using [Lighthouse](https://developer.chrome.com/docs/lighthouse/overview/).