require('esbuild').build({
    entryPoints: ['src/index.ts'],
    bundle: true,
    outfile: 'dist/index.js',
    format: "cjs",
    minify: true,
    platform: "node",
    plugins: [require('esbuild-node-externals').nodeExternalsPlugin()]
}).then(() => console.log(("[compiler]: Built successfully!"))).catch(() => process.exit(1))