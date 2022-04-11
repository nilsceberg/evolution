import { AgentInfo, Frame } from "./types";
import "./World.css";

interface WorldProps {
    frame: Frame;
    agents: AgentInfo[];
}

interface AgentProps {
    info: AgentInfo;
    position: [number, number];
}

function agentColor(genome: string): string {
    let hue = genome
        .split("")
        .map(x => (x.charCodeAt(0) - 97) / 25)
        .map((x, i) => Math.sin(i / 10) * x)
        .reduce((a,b) => a+b, 0);
    return `hsl(${hue * 120 % 360}deg 100% 50%)`;
}

const Agent = (props: AgentProps) => {
    const color = agentColor(props.info[0]);
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

export const World = (props: WorldProps) => {
    return (
        <div className="World">
            <div className="XAxis"/>
            <div className="YAxis"/>
            {props.agents.map((agent, index) => 
                <Agent key={agent[0]} info={agent} position={props.frame[index]}/>
            )}
        </div>
    );
}
