## 2024-05-02 - Large JSON bundle size
**Learning:** The `src/locales/tags/zh-CN.json` file is 1.7M and is imported statically in `src/utils.ts`. This causes the entire JSON file to be bundled into the main chunk, significantly increasing the initial load time and bundle size, even for users who do not use the `zh-CN` locale.
**Action:** Use dynamic import (`import()`) for large JSON files that are not strictly required for the initial render or for all users. We refactored `translateTag` to load the tags lazily, reducing the main bundle size from 2.38MB to 808KB.
