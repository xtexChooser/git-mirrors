/* eslint-env node */

'use strict';

module.exports = {
	root: true,
	extends: [
		'wikimedia/server'
	],
	overrides: [
		{
			files: '**/*.ts',
			plugins: [
				'@typescript-eslint/eslint-plugin'
			],
			parserOptions: {
				parser: '@typescript-eslint/parser',
				sourceType: 'module'
			},
			extends: [
				'plugin:@typescript-eslint/stylistic',
				'plugin:@typescript-eslint/recommended'
			],
			rules: {
				'no-unused-vars': 'off',
				'@typescript-eslint/no-unused-vars': 'error'
			}
		}
	]
};
