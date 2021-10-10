module.exports = {
    pages: {
        index: {
            entry: 'src/pages/index/app.js',
            template: 'public/index.html',
            filename: 'index.html',
            title: 'Index Page',
            chunks: ['chunk-vendors', 'chunk-common', 'index']
        },
        charts: {
            entry: 'src/pages/charts/app.js',
            template: 'public/index.html',
            filename: 'charts/index.html',
            title: 'Charts Page',
            chunks: ['chunk-vendors', 'chunk-common', 'charts']
        }
    }
}