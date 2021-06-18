import * as path from 'path';
import { Configuration } from 'webpack';
import * as devServer from 'webpack-dev-server';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';

export type WebpackConfig = Configuration & {
  devServer?: devServer.Configuration;
};

export const commonConfig: WebpackConfig = {
  mode: 'development',
  devtool: 'inline-source-map',
  context: path.join(__dirname, 'src'),
  entry: ['@babel/polyfill', './index.tsx'],
  output: {
    path: path.join(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          { loader: 'babel-loader' },
          {
            loader: '@linaria/webpack-loader',
            options: {
              sourceMap: process.env.NODE_ENV !== 'production',
            },
          },
          { loader: 'ts-loader' },
        ],
      },
      {
        test: /\.css$/,
        use: [
          { loader: MiniCssExtractPlugin.loader },
          {
            loader: 'css-loader',
            options: {
              sourceMap: process.env.NODE_ENV !== 'production',
            },
          },
        ],
      },
      { 
        test: /\.(png|jpg|gif|svg|jpeg)$/,
        loader: 'file-loader',
        options: {},
      }
    ],
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx'],
    alias: {
      '@': path.resolve(__dirname, 'src'),
      assets: path.resolve(__dirname, 'assets'),
    },
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: 'styles.css',
    }),
  ],
};
