import { PingResponse } from "./response.js";
import fetch from "node-fetch";

export interface PingClient {
  ping(url: string): Promise<PingResponse>;
}

export class HttpPingClient implements PingClient {
  async ping(url: string): Promise<PingResponse> {
    const response = await fetch(url);

    return { status: response.status };
  }
}
