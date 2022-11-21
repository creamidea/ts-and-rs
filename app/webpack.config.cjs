const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    // module: {
    //     rules: [
    //         {
    //             test: /.wasm$/,
    //             type: 'webassembly/sync'
    //         }
    //     ]
    // },
    plugins: [
        // new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".."),
            outDir: path.resolve(__dirname, "hello-rust"),
            // extraArgs: '-d app/hello-rust --target web'
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development',
    experiments: {
        // 因为 wasm 肯定是异步加载
        asyncWebAssembly: false,
        syncWebAssembly: true,
    }
};
