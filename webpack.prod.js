const path = require('path');
const webpack = require('webpack');
const { merge } = require('webpack-merge');
const LicenseWebpackPlugin = require('license-webpack-plugin')
  .LicenseWebpackPlugin;

const base = require('./webpack.base.js');

module.exports = merge(base, {
  mode: 'production',
  devtool: 'hidden-source-map',
  output: {
    publicPath: '/mina/yew-setup-2021/',
  },
  plugins: [
    new webpack.DefinePlugin({ NODE_ENV: '"production"' }),
    new LicenseWebpackPlugin({ perChunkOutput: false }),
  ],
});
