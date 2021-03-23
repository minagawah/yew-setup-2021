const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
  entry: './bootstrap.js',
  output: {
    filename: 'memo.js',
    path: path.resolve(__dirname, 'dist'),
    webassemblyModuleFilename: 'memo.wasm',
  },
  experiments: {
    syncWebAssembly: true,
  },
  stats: {
    colors: true,
  },
  optimization: {
    minimize: false,
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader', 'postcss-loader'],
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './static',
          to: path.resolve(__dirname, 'dist'),
        },
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: '.',
      extraArgs: '--no-typescript',
    }),
  ],
};
