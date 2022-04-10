import { AgentInfo, Frame } from "./types";
import "./World.css";

interface WorldProps {
    frame: Frame;
    agents: AgentInfo[];
}

interface AgentProps {
    position: [number, number];
}

const Agent = (props: AgentProps) => {
    const style = {
        left: props.position[0],
        top: props.position[1],
    };

    return (
        <div className="Agent" style={style}></div>
    )
}

export const World = (props: WorldProps) => {
    return (
        <div className="World">
            {props.agents.map((agent, index) => 
                <Agent key={agent[0]} position={props.frame[index]}/>
            )}
        </div>
    );
}
