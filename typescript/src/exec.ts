import { PingClient } from "./client.js";
import { StatsLogger } from "./logging.js";
import { ResponseHandler } from "./response.js";
import { PingStats } from "./stats.js";

export class Executor {
  constructor(
    private client: PingClient,
    private handler: ResponseHandler,
    private logger: StatsLogger
  ) {}

  async execute(url: string, nbTries: number) {
    const stats = new PingStats();

    for (let i = 0; i < nbTries; i++) {
      const response = await this.client.ping(url);

      this.handler.updateStats(stats, response);
      this.logger.showCurrentStats(stats);
    }

    this.logger.showStatsSummary(stats);
  }
}
