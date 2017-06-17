const path = require('path')
const ExtractTextPlugin = require('extract-text-webpack-plugin')

const config = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, '../dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      { test: /\.css$/, use: ExtractTextPlugin.extract({ use: 'css-loader'}) },
      { test: /\.wav$/, use: 'file-loader' }, // must go to static/audio
      { test: /\.html$/, use: 'html-loader' } // base build folder
    ]
  },
  plugins: [
    new ExtractTextPlugin('styles.css')
  ],
  // node: {
  //   fs: 'empty'
  // }
}

module.exports = config
