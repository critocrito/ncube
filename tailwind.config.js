/* eslint @typescript-eslint/no-var-requires: off, global-require: off */
module.exports = {
  mode: "jit",
  purge: ["./src/ts/**/*.{js,ts,jsx,tsx}", "./resources/public/**/*.html"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        noto: ["Noto", "sans-serif"],
      },

      fontSize: {
        xs: "0.7rem",
        sm: "0.8rem",
        base: "1rem",
        md: "1.25rem",
        lg: "1.563rem",
        xl: "1.953rem",
        "2xl": "2.441rem",
        "3xl": "3.052rem",
      },

      colors: {
        canvas: "#faf8f7",
        bittersweet: "#fd7268",
        sapphire: "#0a2463",
        "fair-pink": "#f8ece8",
        solitude: "#dfe2ec",

        "gray-75": "#474747",
        "gray-25": "#c0bfbf",

        gray: {
          dark: "#474747",
          light: "#c0bfbf",
        },

        success: "#78d397",
        error: "#c94d07",

        youtube: "#e52d27",
        twitter: "#08a0e9",
        http: "#463764",

        washed: {
          red: "#e19f9f",
          purple: "#c4ccea",
          blue: "#a2cae0",
          green: "#b6ddb0",
          yellow: "#e4d976",
        },
      },

      animation: {
        spinner: "spinner 1.5s ease-in-out infinite",
      },

      keyframes: {
        spinner: {
          "0%, 100%": {opacity: 1},
          "60%": {opacity: 0},
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
