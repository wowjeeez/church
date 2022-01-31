function serialize(data: any[]) {
    return data.map(piece => {
        if (typeof piece == "object") {
            return JSON.stringify(piece, null, 2)
        } else {
            return piece
        }
    })
}

function _log(config: {level: "debug" | "info" | "trace" | "silly" | "error", location: string, async: boolean}, ...data: unknown[]) {
    //custom term coloring impl
    const chalk = {
        grey: (...args: unknown[]) => `^9${serialize(args).join(" ")}`,
        white: (...args: unknown[]) => `^7${serialize(args).join(" ")}`,
        red: (...args: unknown[]) => `^1${serialize(args).join(" ")}`,
        cyan: (...args: unknown[]) => `^5${serialize(args).join(" ")}`,
        dim: (...args: unknown[]) => `^3${serialize(args).join(" ")}`,
        blueBright: (...args: unknown[]) => `^4${serialize(args).join(" ")}`
    }
    const date = new Date()
    const fmtDate = `${date.getFullYear()}-${date.getMonth()}-${date.getDay()}-${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`
    switch (config.level) {
        case "debug":
            console.log(chalk.grey(`[${config.location} - DEBUG ${fmtDate}]: ${chalk.white(...data)}`))
            break
        case "error":
            console.log(chalk.red(`[${config.location} - ERROR ${fmtDate}]: ${chalk.white(...data)}`))
            break
        case "info":
            console.log(chalk.cyan(`[${config.location} - INFO ${fmtDate}]: ${chalk.white(...data)}`))
            break
        case "silly":
            console.log(chalk.dim(`[${config.location} - SILLY ${fmtDate}]: ${chalk.white(...data)}`))
            break
        case "trace":
            console.log(chalk.blueBright(`[${config.location} - TRACE ${fmtDate}]: ${chalk.white(...data)}`))
            break
    }
}


export function log(config: {level: "debug" | "info" | "trace" | "silly" | "error", location: string, async: boolean}, ...data: unknown[]) {
   if (config.async) {
       (async  () => _log(config, ...data))()
   } else {
       _log(config, ...data)
   }
}