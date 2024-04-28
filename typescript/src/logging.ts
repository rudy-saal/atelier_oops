import { PingStats } from "./stats.js";

export interface StatsLogger {
  showCurrentStats(stats: PingStats): void;
  showStatsSummary(stats: PingStats): void;
}

export class ConsoleStatsLogger implements StatsLogger {
  showCurrentStats(stats: PingStats) {
    const successPercentage = Math.round(stats.successRate() * 100);
    const message = `SUCCESS RATE : ${successPercentage}%`;

    console.log(message);
  }

  showStatsSummary(stats: PingStats) {
    const successPercentage = Math.round(stats.successRate() * 100);
    const nbTries = stats.total();
    const message = `Successfuly pinged ${successPercentage}% of the requests over ${nbTries} tries.`;

    console.log(message);
  }
}
