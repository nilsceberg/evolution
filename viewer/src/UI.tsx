import { useMemo } from "react";
import { AgentInfo } from "./types";
import "./UI.css";
import { agentColor } from "./util";

interface UIProps {
    agents: AgentInfo[];
    highlight: string;
    onHighlight: (id: string) => void;
}

interface AgentInfoBoxProps {
    info: AgentInfo;
    highlight: boolean;
    onHighlight: (id: string) => void;
}

const clamp = (x: number) => Math.min(1.0, Math.max(0.0, x));

const AgentInfoBox = (props: AgentInfoBoxProps) => {
    const MAX_VALUES = 48;

    const genomeViz = useMemo(() => {
        const values = props.info[1].slice(0, MAX_VALUES)
            .map(v => clamp((v + 1) / 2));
        return values.map((v, i) => <div key={i} style={{backgroundColor: `hsl(0deg 0% ${v*100}%)`}}/>);
    }, [props.info]);

    const color = useMemo(() => agentColor(props.info[1]), [props.info]);

    return (
        <div className={`AgentInfo ${props.highlight ? "highlight" : ""}`} onClick={() => props.onHighlight(props.highlight ? "" : props.info[0])}>
            <div className="AgentId">{props.info[0]}</div>
            <div className="Genome">
                <div className="GenomeColor" style={{backgroundColor: color}}/>
                <div className="GenomeViz">
                    {genomeViz}
                </div>
                ...
            </div>
        </div>
    )
}

export const UI = (props: UIProps) => {
    return (
        <div className="UI">
            {props.agents.map(agent => <AgentInfoBox highlight={agent[0] === props.highlight} onHighlight={props.onHighlight} key={agent[0]} info={agent}/>)}
        </div>
    );
}