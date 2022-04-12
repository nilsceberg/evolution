
export type AgentInfo = [string, number[]];
export type Frame = [number, number][];
export interface Settings {
    title: string;
    world_radius: number;
    time_step: number;
    frame_interval: number;
    generation_time: number;
    zone?: {
        x: number,
        y: number,
        radius: number,
    };
}
