import { PingStats } from "./stats.js";

export type PingResponse = {
  status: number;
};

export class ResponseHandler {
  updateStats(stats: PingStats, response: PingResponse) {
    if (response.status >= 200 && response.status < 300) {
      stats.addSuccess();
    } else {
      stats.addFail();
    }
  }
}
