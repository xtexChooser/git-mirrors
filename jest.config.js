'use strict';

// For a detailed explanation regarding each configuration property, visit:
// https://jestjs.io/docs/en/configuration.html

module.exports = {
	// Automatically clear mock calls and instances between every test
	clearMocks: true,

	// Indicates whether the coverage information should be collected while executing the test
	collectCoverage: true,

	// An array of glob patterns indicating a set of files fo
	//  which coverage information should be collected
	collectCoverageFrom: [
		'cli/**/*.(js|ts)'
	],

	// The directory where Jest should output its coverage files
	coverageDirectory: 'coverage',

	// An array of regexp pattern strings used to skip coverage collection
	coveragePathIgnorePatterns: [
		'/node_modules/',
		'/cli/dist'
	],

	// An object that configures minimum threshold enforcement for coverage results
	coverageThreshold: {
		global: {
			branches: 50,
			functions: 80,
			lines: 80,
			statements: 80
		}
	},

	// An array of file extensions your modules use
	moduleFileExtensions: [
		'js',
		'ts',
		'json'
	],

	testRegex: '(/__tests__/.*|(\\.|/)(test))\\.[jt]sx?$',
	extensionsToTreatAsEsm: [ '.ts' ],
	modulePathIgnorePatterns: [],

	// The paths to modules that run some code to configure or
	// set up the testing environment before each test
	setupFiles: [
		'./jest.setup.js'
	],

	transform: {
		'^.+\\.ts?$': 'ts-jest'
	}
};
