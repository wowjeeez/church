import fs from "fs"
import {promisify} from "util";
import path from "path";
import {ensureFile, rm} from "fs-extra";
const readFile = promisify(fs.readFile)
const readDir = promisify(fs.readdir)
const write = promisify(fs.writeFile)
import * as esbuild from "esbuild"
import parseExpression from "./directives/parser";

interface Config {
    srcDir: string,
    context: "server" | "client"| "shared",
    asyncLogging: boolean
    outDir: string
    entry: string
    type: "core" | "module",
    importLogs: boolean
}

async function readDirectory(dir: string) {
    const curr = await readDir(dir)
    const files: string[] = []
    for (const dirent of curr) {
        if (fs.statSync(path.join(dir, dirent)).isDirectory()) {
            files.push(...(await readDirectory(path.join(dir, dirent))))
        } else {
            files.push(path.join(dir, dirent))
        }
    }
    return files
}

const LOG_FUNCTION_SIGS = ["debug(", "info(", "trace(", "silly(", "error("]

function doesFileDefineCustomLoggerForLevel(src: string, type: typeof LOG_FUNCTION_SIGS[number]) {
    return src.includes(`function ${type}`) || src.includes(`const ${type.replace("(", "")}`) || src.includes(`import {${type.replace("(", "")}}`) || src.includes(`import ${type.replace("(", "")}`)
}


async function createLogger(relToRt: string, cnf: Config) {
    let f = (await readFile(path.join(process.cwd(), cnf.srcDir, relToRt))).toString()
    parseExpression(f, relToRt)
    let imported = false
    LOG_FUNCTION_SIGS.forEach(v => {
        if (f.includes(v) && !doesFileDefineCustomLoggerForLevel(f, v)) {
            if (!imported) {
                f = `import {log as __CHRCH_LOG} from "@church/${cnf.context}"\n${f}`
                imported = true
            }
            f = f.replace(v, `__CHRCH_LOG({location: "${relToRt.replace(/\\/g, "/").slice(1)}", level: "${v.replace("(", "")}", async: ${cnf.asyncLogging ? "true" : "false"}}, `)
        }
    })
    const p = path.join(process.cwd(), "chrch_tmp", relToRt)
    await ensureFile(p)
    const min = (await esbuild.transform(f, {minifySyntax: true, minify: true}))
    min.warnings.forEach(warn => {
        console.warn(`{${warn.location.file}} [${warn.pluginName}]: (${warn.location.line}:${warn.location.column}) ${warn.text} \nNotes:${warn.notes.join("\n")}`)
    })
    await write(p, min.code)
    await write(`${p.replace(".ts", ".js.map")}`, min.map)
    return p
}




async function build(config: Config, entry: string) {
    console.log(`Build entry: ${entry}`)
    const finalOutput = await esbuild.build({entryPoints: [path.join("chrch_tmp", entry)], target: config.context == "server" ? "node16" : "chrome58", outfile: path.join(config.outDir, "bundle.js"), minify: true, minifySyntax: true, minifyWhitespace: true, format: config.context == "server" ? "cjs" : "iife"})
    console.log(finalOutput.outputFiles)
}


async function describeModule(config: Config) {
    const cwd = process.cwd()
    const f = (await readFile(path.join(cwd, config.srcDir, config.entry))).toString()
    const replaced = `${f}\n exports("__CHRCH_GET_TYPE", () => "${config.type}")`
    await write(path.join(cwd, "chrch_tmp", config.entry), replaced)
}

async function compile() {
    const cwd = process.cwd()
    const config: Config = JSON.parse((await readFile(path.join(cwd, "church.json"))).toString())
    const di = path.join(cwd, config.srcDir)
    const dirs = await readDirectory(di)
    await fs.promises.mkdir(path.join(process.cwd(), "chrch_tmp"))
    const pPool: Promise<string>[] = dirs.map(f => createLogger(f.replace(di, ""), config))
    const files = await Promise.all(pPool)
    await describeModule(config)
    await build(config, config.entry)
    await rm(path.join(process.cwd(), "chrch_tmp"), {recursive: true})

}



compile()