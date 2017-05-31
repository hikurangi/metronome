const path = require('path')
const ExtractTextPlugin = require('extract-text-webpack-plugin')

module.exports = {
  entry: './app/index.js',
  output: {
    path: path.resolve(__dirname, '../build-dev'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      { test: /\.css$/, use: ExtractTextPlugin.extract({ use: 'css-loader'}) },
      { test: /\.wav$/, use: 'file-loader' }, // must go to static/audio
      { test: /\.html$/, use: 'html-loader' } // base build folder
    ]
  },
  devServer: {
    contentBase: path.join(__dirname, '../build-dev'),
    compress: true,
    port: 9000
  },
  plugins: [
    new ExtractTextPlugin('styles.css')
  ],
  node: {
    fs: 'empty'
  }
}
