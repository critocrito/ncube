/* eslint @typescript-eslint/no-var-requires: off */
const path = require("path");
const HtmlWebPackPlugin = require("html-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const WebpackExtensionManifestPlugin = require("webpack-extension-manifest-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const baseManifest = require("./manifest");
const pkg = require("./package.json");

module.exports = {
  entry: {
    popup: "./src/ts/web-extension.tsx",
  },

  output: {
    path: path.resolve(__dirname, "target/web-ext"),
    filename: "[name].[chunkhash:8].js",
    clean: true,
  },

  devtool: "source-map",

  performance: {
    hints: false,
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: "babel-loader",
          },
        ],
        exclude: /node_modules/,
      },
      {
        test: /\.html$/,
        use: [
          {
            loader: "html-loader",
          },
        ],
      },
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
      {
        test: /\.(jpe?g|png|gif|svg|woff|woff2|eot|ttf)$/i,
        use: [
          {
            loader: "url-loader",
            options: {
              limit: 30000,
            },
          },
        ],
      },
    ],
  },

  resolve: {
    extensions: [".ts", ".tsx", ".json", ".js", ".jsx"],
  },

  plugins: [
    new CopyPlugin({
      patterns: [{from: "resources/icons", to: "icons"}],
    }),
    new HtmlWebPackPlugin({
      template: path.resolve(__dirname, "resources/public/popup.html"),
      filename: "./popup.html",
    }),
    new MiniCssExtractPlugin({
      filename: "[name].[chunkhash:8].css",
      chunkFilename: "[name].[chunkhash:8].css",
    }),
    new WebpackExtensionManifestPlugin({
      config: {
        base: baseManifest,
        extend: {
          description: pkg.description,
          homepage_url: pkg.homepage,
        },
      },
    }),
  ],
};
