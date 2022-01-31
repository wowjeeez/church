const REXP = /\/\/\s*@church\(.*\):.+/gm

const isVariable = (sym: string) => /^[a-zA-Z ]*$/gm.test(sym)
const replaceModifier = (sym: string) => sym.startsWith("!") ? [sym.replace("!", ""), true] : [sym, false]

function evalExpr(expr: string, file: string) {
        const splitBy = (by: string) => expr.split(by).map(v => v.trim()).concat(by)
        let res: string[] = []
        if (expr.includes("==")) {
                res = splitBy("==")
        } else if (expr.includes("<")) {
                res = splitBy("<")
        } else if (expr.includes(">")) {
                res = splitBy("<")
        } else if (expr.includes("!=")) {
                res = splitBy("!=")
        } else {
                console.error(`${file}: Invalid expression in compiler directive: ${expr}`)
        }
        const [left, right, action] = res
        console.log(isVariable(left))

}

export default function parseExpression(srcFile: string, fname: string) {
        const matches = (srcFile.match(REXP) || []).filter(v => v)
        matches.forEach(expr =>  {
                const [dir, cmd] = expr.split(":").map(v => v.trim().replace("//", ""))
                const sanitizedDir = dir.replace("@church(", "")
                if (sanitizedDir.includes("cfg(")) {
                        const inside = sanitizedDir.match(/\(([^\)]+)\)/gm)[0]
                        if (inside) {
                                const matched = inside.slice(1, inside.length - 1)
                                evalExpr(matched, fname)
                        }
                }
        })

}