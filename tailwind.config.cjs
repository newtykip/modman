const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config}*/
const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],

	safelist: [
		{
			pattern: /^grid-cols-\d+$/
		}
	],

	theme: {
		extend: {
			fontFamily: {
				sans: ['Inter', ...defaultTheme.fontFamily.sans],
				warming: ['Warming']
			},
			colors: {
				titlebar: '#050818',
				'titlebar-highlight': '#1b2533'
			}
		}
	},

	plugins: [],
};

module.exports = config;
