module.exports = {
  plugins: {
    "postcss-import": {},
    "postcss-insert": {},
    "postcss-custom-properties": { preserve: false },
    "postcss-preset-env": { browsers: "> 0.2%" },
    autoprefixer: {}
  }
};
