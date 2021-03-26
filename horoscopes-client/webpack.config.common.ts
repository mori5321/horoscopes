import * as path from "path"
import {Configuration} from "webpack";
type WebpackConfig = Configuration & { devServer?: devServer.Configuration }
import * as devServer from "webpack-dev-server";

const commonConfig: WebpackConfig  = {
  mode: "development",
  devtool: "inline-source-map",
  context: path.join(__dirname, "src"),
  entry: "./index.tsx",
  output: {
    path: path.join(__dirname, "dist"),
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".jsx"]
  },
}

export { commonConfig, WebpackConfig };

