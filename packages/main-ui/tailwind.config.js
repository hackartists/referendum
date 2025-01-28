/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  plugins: [require("daisyui")],

  daisyui: {
    themes: false,
    darkTheme: "dark",
    base: true,
    styled: true,
    utils: true,
    prefix: "",
    logs: true,
    themeRoot: ":root",
  },
};
