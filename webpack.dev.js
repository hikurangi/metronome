const path = require('path')
const ExtractTextPlugin = require('extract-text-webpack-plugin')

module.exports = {
  entry: './app/index.js',
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
  devServer: {
    contentBase: path.join(__dirname, "dist"),
    compress: true,
    port: 9000
  },
  plugins: [
    new ExtractTextPlugin('styles.css')
  ]
}
