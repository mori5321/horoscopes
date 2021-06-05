import { commonConfig } from './webpack.config.common';
import { merge } from 'webpack-merge';
import * as path from 'path';
import { WebpackConfig } from './webpack.config.common';

const devConfig: WebpackConfig = merge(commonConfig, {
  mode: 'development',
  devtool: 'inline-source-map',
  devServer: {
    port: 8080,
    contentBase: path.join(__dirname, 'dist'),
  },
});

export default devConfig;
