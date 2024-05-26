module.exports = {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'dark-beige': '#F87C6B',
        'mid-beige': '#FECAAC',
        'light-beige': '#D4D4AA',
        'japanese-light-blue': '#B5C6F5',
        'candy-pink': '#D4AAC8',
        'moss-green': '#D4D4AA',
        'foggy-gray': "#D9D9D9",
        'bento-red': "#FE452A",
        'forest-green': '#013000'
      }
    },
  },
  plugins: [
    // default prefix is "ui"
    require("@kobalte/tailwindcss"),
  ],
};
