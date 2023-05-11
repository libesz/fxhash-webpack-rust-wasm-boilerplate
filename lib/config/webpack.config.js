const path = require("path")
const webpack = require("webpack")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./project/src/index.js",
  output: {
    path: path.resolve(__dirname, "../../dist"),
    filename: "bundle.js",
    clean: true
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./project/public/index.html",
      inject: "body",
      publicPath: "./",
      minify: false,
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../../project")
  }),
  ],
  experiments: {
    asyncWebAssembly: true
}
}
