
export function agentColor(genome: number[]): string {
    let hue = genome
        .map((x, i) => Math.sin(i / 10) * x)
        .reduce((a,b) => a+b, 0);
    return `hsl(${hue * 20 % 360}deg 100% 50%)`;
}
