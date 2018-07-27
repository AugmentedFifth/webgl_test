const path = require("path");
const webpack = require("webpack");

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
        jsonpScriptType: "module",
    },
    mode: "none",
    plugins: [
        new webpack.optimize.ModuleConcatenationPlugin(),
    ],
    optimization: {
        noEmitOnErrors: true,
        namedModules: true,
        namedChunks: true,
    },
    target: "web",
    performance: {
        hints: "warning",
    }
};
