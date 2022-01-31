import chalk from "chalk";
export default function log(config: {level: "debug" | "info" | "trace" | "silly" | "error", location: string, async: boolean}, ...data: unknown[]) {
    if (config.async) {
        (async () => _log(config, ...data))()
    } else {
        _log(config, ...data)
    }
}

function _log(config: {level: "debug" | "info" | "trace" | "silly" | "error", location: string, async: boolean}, ...data: unknown[]) {
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