
export type AgentInfo = [string, number[]];
export type Frame = [number, number][];
export interface Settings {
    title: string;
    world_radius: number;
    zone?: {
        x: number,
        y: number,
        radius: number,
    };
}
