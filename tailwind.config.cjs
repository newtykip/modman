const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import("tailwindcss").Config}*/
const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],

	safelist: [
		{
			pattern: /^grid-cols-\d+$/
		},
		{
			pattern: /^bg-.*$/
		}
	],

	theme: {
		extend: {
			fontFamily: {
				sans: ["Karla", ...defaultTheme.fontFamily.sans]
			},
			colors: {
				titlebar: "#050818",
				"titlebar-highlight": "#1b2533"
			}
		}
	},

	plugins: [
		require('@tailwindcss/forms')
	],
};

module.exports = config;
