/** @type {import('@commitlint/types').UserConfig} */
export default {
	extends: ["@commitlint/config-conventional"],
	rules: {
		"body-max-line-length": [2, "always", 200],
		"footer-max-line-length": [2, "always", 200],
		"type-enum": [
			2,
			"always",
			["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"]
		],
		"scope-case": [2, "always", "lower-case"],
		"subject-case": [2, "never", ["upper-case", "pascal-case"]],
		"subject-empty": [2, "never"],
		"subject-full-stop": [2, "never", "."]
	}
};
