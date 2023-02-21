module.exports = {
  env: { browser: true, es2021: true, node: true },
  extends: [
    'standard-with-typescript',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'prettier'
  ],
  plugins: ['simple-import-sort', 'unused-imports'],
  overrides: [],
  parserOptions: {
    sourceType: 'module',
    project: ['./client/tsconfig.json']
  },
  rules: {
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': 'off',
    'unused-imports/no-unused-imports': 'error',
    'unused-imports/no-unused-vars': [
      'warn',
      {
        vars: 'all',
        varsIgnorePattern: '^_',
        args: 'after-used',
        argsIgnorePattern: '^_'
      }
    ],
    '@typescript-eslint/prefer-nullish-coalescing': 'off',
    '@typescript-eslint/strict-boolean-expressions': 'off',
    '@typescript-eslint/no-base-to-string': 'off'
  }
};
