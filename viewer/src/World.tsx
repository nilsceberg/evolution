import { useMemo } from "react";
import { AgentInfo, Frame, Settings } from "./types";
import { agentColor } from "./util";
import "./World.css";

interface WorldProps {
    frame: Frame;
    agents: AgentInfo[];
    settings: Settings;
    highlight: string;
    onHighlight: (id: string) => void;
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
    return (
        <div className="World">
            <Edge radius={props.settings.radius}/>
            <div className="XAxis"/>
            {props.settings.zone ? <Zone {...props.settings.zone}/> : null}
            <div className="YAxis"/>
            {props.agents.map((agent, index) => 
                <Agent key={agent[0]} onHighlight={props.onHighlight} info={agent} highlight={agent[0] === props.highlight} position={props.frame[index]}/>
            )}
            <h1>{props.settings.title}</h1>
        </div>
    );
}
