import { PingStats } from "./stats.js";

describe(PingStats.name, () => {
  describe("initially", () => {
    it("has success rate of 0", () => {
      const stats = new PingStats();
      expect(stats.successRate()).toEqual(0);
    });

    it("has a total of 0", () => {
      const stats = new PingStats();
      expect(stats.total()).toEqual(0);
    });
  });

  describe("given a single success", () => {
    const stats = new PingStats();
    stats.addSuccess();

    it("has a success rate of 1", () => {
      expect(stats.successRate()).toEqual(1);
    });

    it("has a total of 1", () => {
      expect(stats.total()).toEqual(1);
    });
  });

  describe("given a single fail", () => {
    const stats = new PingStats();
    stats.addFail();

    it("has a success rate of 0", () => {
      expect(stats.successRate()).toEqual(0);
    });

    it("has a total of 1", () => {
      expect(stats.total()).toEqual(1);
    });
  });

  describe("given both fails and successes", () => {
    const stats = new PingStats();
    stats.addFail();
    stats.addSuccess();
    stats.addFail();

    it("has the correct success rate", () => {
      expect(stats.successRate()).toEqual(1 / 3);
    });

    it("has the correct total", () => {
      expect(stats.total()).toEqual(3);
    });
  });
});
