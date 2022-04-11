import { useMemo } from "react";
import { AgentInfo, Frame, Settings } from "./types";
import "./World.css";

interface WorldProps {
    frame: Frame;
    agents: AgentInfo[];
    settings: Settings;
}

interface AgentProps {
    info: AgentInfo;
    position: [number, number];
}

function agentColor(genome: number[]): string {
    let hue = genome
        .map((x, i) => Math.sin(i / 10) * x)
        .reduce((a,b) => a+b, 0);
    return `hsl(${hue * 120 % 360}deg 100% 50%)`;
}

const Agent = (props: AgentProps) => {
    const color = useMemo(() => agentColor(props.info[1]), [props.info[1]]);
    const style = {
        marginLeft: props.position[0] - 4,
        marginTop: props.position[1] - 4,
        backgroundColor: color,
        boxShadow: `0px 0px 20px 5px ${color}`,
    };

    return (
        <div className="Agent" style={style}></div>
    )
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

export const World = (props: WorldProps) => {
    return (
        <div className="World">
            <Edge radius={props.settings.radius}/>
            <div className="XAxis"/>
            <div className="YAxis"/>
            {props.agents.map((agent, index) => 
                <Agent key={agent[0]} info={agent} position={props.frame[index]}/>
            )}
        </div>
    );
}
