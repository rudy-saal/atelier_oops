export class PingStats {
  private nbSuccesses: number = 0;
  private nbFails: number = 0;

  addSuccess() {
    this.nbSuccesses += 1;
  }

  addFail() {
    this.nbFails += 1;
  }

  successRate(): number {
    if (this.total() === 0) {
      return 0;
    }

    return this.nbSuccesses / this.total();
  }

  total(): number {
    return this.nbSuccesses + this.nbFails;
  }
}
