/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}
