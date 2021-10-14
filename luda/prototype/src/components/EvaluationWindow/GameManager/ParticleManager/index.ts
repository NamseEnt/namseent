import Particle from "./Particle";

export default class ParticleManager {
  private particles: Particle[] = [];
  private nextParticles: Particle[] = [];
  private lastTickTime: number = Date.now();

  public tick(currentTime: number) {
    const dt = (currentTime - this.lastTickTime) / 1000;
    this.lastTickTime = currentTime;

    this.particles = [
      ...this.particles.filter((particle) => particle.tick(dt)),
      ...this.nextParticles,
    ];

    this.nextParticles = [];
  }

  public render(context: CanvasRenderingContext2D) {
    this.particles.forEach((particle) => particle.render(context));
  }

  public addParticle(particle: Particle) {
    this.nextParticles.push(particle);
  }
}
