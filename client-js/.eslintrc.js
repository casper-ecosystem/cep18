module.exports = {
  env: { browser: true, es2021: true, node: true },
  extends: [
    'airbnb-base',
    'airbnb-typescript/base',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'plugin:eslint-comments/recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'prettier'
  ],
  plugins: [
    '@typescript-eslint',
    'eslint-comments',
    'jest',
    'promise',
    'unicorn',
    'simple-import-sort',
    'unused-imports'
  ],
  overrides: [
    {
      files: ['*.ts'], // Your TypeScript files extension
      // As mentioned in the comments, you should extend TypeScript plugins here,
      // instead of extending them outside the `overrides`.
      // If you don't want to extend any rules, you don't need an `extends` attribute.
      extends: ['plugin:@typescript-eslint/recommended'],
      parserOptions: {
        sourceType: 'module',
        tsconfigRootDir: __dirname,
        project: ['./tsconfig.json']
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
        'simple-import-sort/imports': 'error',
        'simple-import-sort/exports': 'error',
        'import/first': 'error',
        'import/newline-after-import': 'error',
        'import/no-duplicates': 'error',
        '@typescript-eslint/prefer-nullish-coalescing': 'off',
        '@typescript-eslint/strict-boolean-expressions': 'off',
        '@typescript-eslint/no-base-to-string': 'off',
        'no-return-await': 'off',
        '@typescript-eslint/return-await': 'warn',
        'no-console': ['error', { allow: ['warn'] }],
        '@typescript-eslint/naming-convention': [
          'error',
          {
            selector: 'enum',
            format: ['UPPER_CASE'],
            leadingUnderscore: 'allow',
            trailingUnderscore: 'allow'
          }
        ]
      },
      overrides: [
        {
          files: ['*.test.ts'],
          rules: {
            '@typescript-eslint/no-unsafe-member-access': 'off',
            '@typescript-eslint/no-explicit-any': 'off'
          }
        },
        {
          files: ['examples/*.ts'],
          rules: {
            'no-console': 'off',
            'import/no-extraneous-dependencies': 'off'
          }
        }
      ]
    }
  ]
};
