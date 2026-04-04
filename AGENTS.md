# AGENTS.md

Repository guide for agentic edits in this workspace.

## Scope
- This repo's app lives in `client/`.
- Run project scripts from `client/` unless noted otherwise.
- There is no existing `AGENTS.md`, `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` in this checkout.
- If those files are added later, they should override anything here.

## Package Manager
- `client/package.json` declares `yarn@4.3.1`, but this checkout currently has `client/package-lock.json` and no `yarn.lock`.
- `yarn` currently errors in this workspace because the lockfile does not match.
- Prefer the package manager that works in the current checkout; in practice, `npm` is the safer default here.

## Common Commands
- Install deps: `npm install`
- Start dev server: `npm start`
- Build production bundle: `npm run build`
- Run tests in watch mode: `npm test`
- Regenerate GraphQL artifacts: `npm run compile`
- There is no dedicated `lint` script in `package.json`.

## Test Commands
- Run all tests once: `CI=true npm test -- --watchAll=false`
- Run a single test file: `CI=true npm test -- --runInBand src/App.test.tsx`
- Run a single test by name: `CI=true npm test -- --runInBand -t "test name"`
- When debugging a failing test, prefer `--runInBand` to avoid watch/process noise.
- Jest is configured through Create React App, so tests live under `src/` and should use `*.test.tsx` or `*.test.ts` naming.

## Validation Expectations
- For UI changes, run at least `npm run build` and the smallest relevant test target.
- For GraphQL-related edits, also run `npm run compile`.
- Because there is no standalone lint script, treat CRA build/test output as the primary lint gate.

## File Layout
- `src/components/atoms`: small reusable UI pieces.
- `src/components/organisms`: larger composed UI blocks.
- `src/components/layouts`: page shells and route wrappers.
- `src/components/pages`: route-level screens and their local subcomponents.
- `src/common`: shared hooks, stores, routes, constants, and types.
- `src/__generated__`: GraphQL codegen output. Do not edit by hand.

## TypeScript Rules
- TypeScript is strict (`strict: true` in `tsconfig.json`), so avoid introducing `any` unless there is no practical alternative.
- Prefer explicit types for helper return values, mutation/query variables, and exported props when it improves clarity.
- Use type aliases for object-shaped state and props when the shape is local to a module.
- Reuse generated GraphQL types from `src/__generated__/graphql.ts` instead of re-declaring server types.
- Keep nullability honest. Do not erase `null`/`undefined` with assertions unless the value is truly guaranteed.

## React Rules
- Prefer function components.
- Name components in `PascalCase` and hooks with a `use` prefix.
- Keep route-level components and reusable subcomponents separate.
- Use early returns over deeply nested conditionals.
- Preserve the current controlled-component pattern for inputs and forms.
- Use `useEffect`, `useMemo`, `useCallback`, and `useLayoutEffect` only when there is a real need; do not add them preemptively.
- When a component needs local derived state, keep it as close to the consumer as possible.

## Imports
- Follow the local grouping pattern: external packages first, then internal modules, then styles.
- Prefer relative imports; no path alias is configured.
- Keep import lists tidy and remove unused imports immediately.
- Generated GraphQL helpers should be imported from `src/__generated__` paths, not copied into feature files.

## Formatting
- Match the surrounding file style if you touch an existing file.
- Current codebase convention is mostly 2-space indentation.
- Use semicolons in new code.
- Prefer double quotes for imports and strings in new files, unless a file already uses a different local style.
- Keep JSX readable with one prop per line once a call/site becomes dense.
- Avoid unnecessary blank lines and keep functions compact.
- Do not reformat untouched files as part of unrelated edits.
- Keep indentation, spacing, and quote style consistent within the file you are modifying.

## Styling and UI
- Most UI styling is done with CSS modules named `*.module.css`.
- Access classes through the imported `classes` object or destructure only when it improves readability.
- Prefer local component state for purely visual concerns; do not introduce global state without a clear need.
- Preserve existing layout and interaction patterns unless the task explicitly changes them.
- SVG/icon helpers live close to the components that use them; keep that pattern when adding new icons.

## Naming
- Components: `PascalCase` file exports such as `MainPage`, `Button`, `UserBar`.
- Hooks: `useLogin`, `useNotify`, `useTelegramInitData`.
- Stores: `useUserStore`, `useUIStore`.
- Constants and enums: `UPPER_SNAKE_CASE` or `PATHS`-style enums.
- CSS module classes: `camelCase` keys accessed through the imported `classes` object.
- Barrel files usually re-export sibling modules through `index.ts`.

## State and Stores
- Zustand stores live in `src/common/store`.
- Prefer small focused actions that update one concern at a time.
- Keep state updates immutable and local to the store action.
- When a store field is derived from backend data, preserve the server shape unless there is a strong reason to normalize it.
- Avoid widening store types to `any`; model the shape explicitly and keep setter signatures narrow.

## Apollo and Async
- Apollo hooks are used directly in components and custom hooks; keep queries/mutations close to the consumer.
- Use `skip` for missing IDs or prerequisites instead of sending empty variables.
- Check returned mutation data before chaining follow-up work like `refetch()`.
- Prefer `async`/`await` or a single promise chain over deeply nested `.then()` blocks when you touch async code.
- If a request updates local storage or a store, keep the side effect near the successful response handling.

## GraphQL
- GraphQL documents are colocated in hooks/components and wrapped with `gql(/* GraphQl */ \`...\`)`.
- Keep query/mutation names descriptive and aligned with the backend operation.
- Regenerate codegen output after changing schema or documents with `npm run compile`.
- Never hand-edit `src/__generated__/` files.

## Error Handling
- Prefer checking query `error` and mutation results explicitly.
- Do not swallow promise rejections.
- Use early exits for missing IDs, empty variables, or skipped queries.
- If async work can fail silently in the UI, surface a visible state or at least preserve the existing error flow.

## Tests
- `src/setupTests.ts` loads `@testing-library/jest-dom` matchers.
- Prefer React Testing Library for component tests.
- Test user-visible behavior, not implementation details.
- Keep test names descriptive and stable.
- For smoke tests tied to app bootstrap, keep the assertions aligned with the actual router/provider setup.
- When fixing a bug, add or update the smallest test that would have caught the regression.

## Comments And Docs
- Prefer clear code over explanatory comments.
- Add brief comments only when logic is genuinely non-obvious or the surrounding code would otherwise be hard to parse.
- Keep doc or README updates focused on commands and behavior changes that affect other agents.

## Legacy Quirks
- Some files have inconsistent casing or typos in names, such as `tixt-field` or other legacy paths.
- Do not rename those files unless the task explicitly requires it.
- The codebase has a mix of quote styles in older files; follow the local file instead of normalizing the whole repo.
- You may encounter older `any` usage or loose null handling; tighten only the code you touch.

## React App Conventions
- The app uses Create React App (`react-scripts`) and React Router.
- `src/App.tsx` wires Apollo, routing, and app-wide providers.
- `src/index.tsx` is the bootstrap entry point and should stay minimal.
- `src/common/routes.tsx` centralizes route definitions.

## Editing Guidance
- Make the smallest correct change.
- Do not rename generated, public, or route-linked files unless needed for the task.
- Preserve existing behavior unless the user asked for a change.
- When in doubt, follow the nearest neighboring file rather than imposing a new style.

## Repository Notes
- Some existing files contain legacy inconsistencies such as mixed quote styles, typos in filenames, and occasional `any` usage.
- Do not do broad cleanup unless the task requires it.
- Prefer targeted fixes over style-only refactors.
- If you must touch generated or legacy code, keep the diff minimal.

## Workflow Notes
- Read the nearest neighboring file before editing a feature area.
- Prefer the smallest diff that solves the problem completely.
- If a change affects shared helpers, search for nearby call sites before refactoring broadly.
- If you add a new test file, keep it next to the code it covers and follow the existing `*.test.tsx` naming.
- If you add new GraphQL documents, regenerate generated files before finishing.
