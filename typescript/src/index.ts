import { parseArgs } from "./cli.js";
import { HttpPingClient } from "./client.js";
import { Executor } from "./exec.js";
import { ConsoleStatsLogger } from "./logging.js";
import { ResponseHandler } from "./response.js";

const client = new HttpPingClient();
const handler = new ResponseHandler();
const logger = new ConsoleStatsLogger();
const executor = new Executor(client, handler, logger);

const args = await parseArgs();

executor.execute(args.url, args.nbTries);
