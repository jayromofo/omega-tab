# fdm4 Launch

This is a "launch page" or home page, intended to replace FDM4 Emplyoee's new tab page with a sleek launch page where you can launch all the tools we use internally, search with your favorite search engine, reach jira ticket pages quickly, and more.

Below is "getting started info" provided by the Vue installer. It's being kept for posterity.

## TODO
Need to add unit and end-to-end testing

## Note about development
Be sure to use the command `bun dev:all` to run, otherwise the tailwind build step won't kick in.

---

This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
bun install
```

### Compile and Hot-Reload for Development

```sh
bun dev
```

### Type-Check, Compile and Minify for Production

```sh
bun run build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
bun test:unit
```

### Run End-to-End Tests with [Cypress](https://www.cypress.io/)

```sh
bun test:e2e:dev
```

This runs the end-to-end tests against the Vite development server.
It is much faster than the production build.

But it's still recommended to test the production build with `test:e2e` before deploying (e.g. in CI environments):

```sh
bun run build
bun test:e2e
```
