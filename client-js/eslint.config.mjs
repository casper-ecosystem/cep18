import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import prettier from 'eslint-config-prettier';
import importPlugin from 'eslint-plugin-import';

export default [
  js.configs.recommended,
  {
    files: ['**/*.ts'],
    languageOptions: {
      parser: tsParser,
      globals: {
        console: true, // ✅ Allow console in general
        process: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly'
      }
    },
    plugins: { '@typescript-eslint': ts, import: importPlugin },
    rules: {
      // TypeScript-specific rules
      '@typescript-eslint/no-explicit-any': 'warn', // Warn about explicit 'any' usage
      '@typescript-eslint/no-unused-vars': 'error', // Enforce unused variable checks for TypeScript
      'no-unused-vars': 'warn', // Disable the base ESLint rule to avoid conflict

      // General JavaScript/TypeScript best practices
      'no-console': ['error', { allow: ['warn', 'error'] }], // ❌ Disallow log, ✅ Allow warn and arror
      'no-lonely-if': 'warn', // Warn about 'if' statements that could be merged

      // Naming conventions
      '@typescript-eslint/naming-convention': [
        'error',
        {
          selector: 'enum',
          format: ['UPPER_CASE'],
          leadingUnderscore: 'allow',
          trailingUnderscore: 'allow'
        }
      ],
      'import/order': [
        'error',
        {
          groups: [
            ['builtin', 'external'], // Ensure built-in modules come before external ones
            ['internal', 'parent', 'sibling'], // Internal imports should be after built-in and external
            ['index'] // Index files should come last
          ],
          alphabetize: {
            order: 'asc', // Optionally alphabetize imports within each group
            caseInsensitive: true // Case-insensitive ordering
          }
        }
      ]
    },
    ignores: ['eslint.config.js', 'dist'] // Ignore config files and dist folder
  },
  {
    files: ['examples/*.ts', 'tests/*.ts'], // Apply special rules for examples
    rules: {
      'no-console': 'off' // Allow console in examples
    }
  },
  prettier // Use Prettier config to avoid conflicts with ESLint
];
