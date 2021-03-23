const path = require('path');
const webpack = require('webpack');
const { merge } = require('webpack-merge');
const LicenseWebpackPlugin = require('license-webpack-plugin')
  .LicenseWebpackPlugin;

const base = require('./webpack.base.js');

module.exports = merge(base, {
  mode: 'production',
  devtool: 'hidden-source-map',
  plugins: [
    new webpack.DefinePlugin({ NODE_ENV: '"production"' }),
    new LicenseWebpackPlugin({ perChunkOutput: false }),
  ],
});
