module.exports =  {
    entry: __dirname + '/src/sample.js', 
    output:  {
        path:  __dirname +  '', 
        filename:  'bundle.js'
    }, 
    module: {
        loaders :  [
            {
                test :  /\.js$/,  
                exclude :  /node_modules/,  
                loader :  "babel-loader",  
                query : {
                    presets :  ['es2015'],
                    plugins :  ['transform-flow-strip-types']
                }
            }
        ]
    }
}
