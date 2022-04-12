import { useEffect, useMemo, useRef } from "react";
import { AgentInfo, Frame, Settings } from "./types";
import { agentColor } from "./util";
import "./World.css";

interface WorldProps {
    frame: Frame;
    agents: AgentInfo[];
    settings: Settings;
    highlight: string;
    onHighlight: (id: string) => void;
    startTime: Date;
    simulationTime: number;
}

interface AgentProps {
    info: AgentInfo;
    highlight: boolean;
    position: [number, number];
    onHighlight: (id: string) => void;
}

const Agent = (props: AgentProps) => {
    const color = useMemo(() => agentColor(props.info[1]), [props.info[1]]);
    const style = {
        marginLeft: props.position[0] - 4,
        marginTop: props.position[1] - 4,
        backgroundColor: color,
        boxShadow: props.highlight ? `0px 0px 20px 20px #bbc` : "none", //`0px 0px 20px 5px ${color}`,
    };

    return (
        <div className="Agent" style={style} onClick={() => props.onHighlight(props.info[0])}></div>
    );
}

const Edge = ({ radius }: { radius: number }) => {
    const style = {
        width: radius*2,
        height: radius*2,
        borderRadius: radius,
        marginLeft: -radius,
        marginTop: -radius,
    };
    return <div className="Edge" style={style}/>;
}

const Zone = ({ x, y, radius }: { x: number, y: number, radius: number }) => {
    const style = {
        width: radius*2,
        height: radius*2,
        borderRadius: radius,
        marginLeft: -radius + x,
        marginTop: -radius + y,
    };
    return <div className="Zone" style={style}/>;
}


export const World = (props: WorldProps) => {
    /*const timerRef = useRef<HTMLHeadingElement>(null);

    useEffect(() => {
        const interval = setInterval(() => {
            if (timerRef.current) {
                const now = new Date().getTime() / 1000;
                const start = props.startTime.getTime() / 1000;
                const frame_time = props.settings.frame_interval / 1000;
                const time_steps_per_second = props.settings.time_step / frame_time;
                const generation_real_time = props.settings.generation_time / time_steps_per_second;
                console.log(generation_real_time);
                timerRef.current.innerText = `${(generation_real_time - (now - start)).toFixed(1)} s`;
            }
        }, 50);
        return () => {
            clearInterval(interval);
        }
    }, [props.startTime]);*/

    return (
        <div className="World">
            <Edge radius={props.settings.world_radius}/>
            <div className="XAxis"/>
            {props.settings.zone ? <Zone {...props.settings.zone}/> : null}
            <div className="YAxis"/>
            {props.agents.map((agent, index) => 
                <Agent key={agent[0]} onHighlight={props.onHighlight} info={agent} highlight={agent[0] === props.highlight} position={props.frame[index]}/>
            )}
            <h1 className="Title">{props.settings.title}</h1>
            <h1 className="Timer">{Math.max(props.settings.generation_time - props.simulationTime, 0).toFixed(1)} tu</h1>
        </div>
    );
}
