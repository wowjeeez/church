require('esbuild').build({
    entryPoints: ['src/index.ts'],
    bundle: true,
    outfile: 'dist/bundle.js',
    format: "cjs",
    minify: false,
    platform: "node",
    keepNames: true,
    sourcemap: "external"
}).then(() => console.log(("[server]: Built successfully!"))).catch(() => process.exit(1))