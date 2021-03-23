const path = require('path');
const webpack = require('webpack');
const { merge } = require('webpack-merge');

const base = require('./webpack.base.js');

module.exports = merge(base, {
  mode: 'development',
  devtool: 'inline-source-map',
  watch: true,
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    hot: true,
    port: 8080,
    writeToDisk: true, // Prevents 404 for: /assets
  },
  plugins: [
    new webpack.DefinePlugin({
      NODE_ENV: '"development"',
    }),
  ],
});
