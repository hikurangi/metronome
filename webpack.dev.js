const path = require('path')
const ExtractTextPlugin = require('extract-text-webpack-plugin')
import 'css/styles.css'

module.exports = {
  entry: './app/index.js',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist')
  },
  module: {
    rules: [{
      test: /\.css$/,
      use: ExtractTextPlugin.extract({
        use: 'css-loader'
      })
    }]
  },
  plugins: [
    new ExtractTextPlugin('styles.css')
  ]
}
