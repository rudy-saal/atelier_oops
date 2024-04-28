import yargs from "yargs/yargs";
import { hideBin } from "yargs/helpers";

export type Args = {
  url: string;
  nbTries: number;
};

export async function parseArgs(): Promise<Args> {
  const parser = yargs(hideBin(process.argv))
    .option("url", {
      alias: "u",
      description: "URL to ping",
      type: "string",
    })
    .option("tries", {
      alias: "t",
      description: "Number of tries",
      type: "number",
      default: 20,
    })
    .demandOption(["url"]);

  const args = await parser.argv;

  return {
    url: args.url!,
    nbTries: args.tries,
  };
}
