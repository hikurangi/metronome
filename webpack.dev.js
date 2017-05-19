const path = require('path')
const ExtractTextPlugin = require('extract-text-webpack-plugin')

module.exports = {
  context: path.resolve(__dirname, './app'),
  entry: {
    app: './index.js'
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      { test: /\.css$/, use: ExtractTextPlugin.extract({ use: 'css-loader'}) },
      { test: /\.wav$/, use: 'file-loader' },
      { test: /\.html$/, use: 'html-loader' }
    ]
  },
  plugins: [
    new ExtractTextPlugin('styles.css')
  ]
}
