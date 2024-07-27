module.exports = {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'orbit-blue': '#3A9DFB',
        'dark-beige': '#F87C6B',
        'mid-beige': '#FECAAC',
        'japanese-light-blue': '#B5C6F5',
        'candy-pink': '#D4AAC8',
        'foggy-gray': "#D9D9D9",
        'bento-red': "#FE452A",
        'goose-green': '#D4D4AA',
        'moss-green': '#D4D4AA',
        'forest-green': '#013000'
      }
    },
  },
  plugins: [
    // default prefix is "ui"
    require("@kobalte/tailwindcss"),
  ],
};
