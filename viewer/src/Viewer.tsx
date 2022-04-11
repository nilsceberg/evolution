import { useEffect, useState } from 'react';
import { AgentInfo, Frame, Settings } from './types';
import './Viewer.css';
import { World } from "./World";
import { UI } from "./UI";

interface ViewerProps {
    url: string,
}

enum Status {
    CONNECTING,
    RECONNECTING,
    CONNECTED,
    DISCONNECTED,
}

export const Viewer = (props: ViewerProps) => {
    const [status, setStatus] = useState(Status.CONNECTING);
    const [disconnected, setDisconnected] = useState(false);

    const [agents, setAgents] = useState<AgentInfo[]>([]);
    const [frame, setFrame] = useState<Frame>([]);
    const [settings, setSettings] = useState<Settings>({
        radius: 0,
    });

    const [highlight, setHighlight] = useState<string>("");

    const onClear = () => {
        setAgents([]);
        setFrame([]);
    };

    const onSpawn = (newAgents: AgentInfo[]) => {
        setFrame([...frame, ...newAgents.map(agent => [0, 0] as [number, number])]);
        setAgents([...agents, ...newAgents]);
    }

    const onKill = (indices: number[]) => {
        setAgents(agents.filter((_, i) => i in indices));
        setFrame(frame.filter((_, i) => i in indices));
    }

    const onFrame = (frame: Frame) => {
        setFrame(frame);
    }

    const onSettings = (settings: Settings) => {
        setSettings(settings);
    }

    useEffect(() => {
        if (disconnected) {
            setStatus(Status.DISCONNECTED);
            console.log("reconnecting in 1 second...")
            const timeout = setTimeout(() => {
                console.log("reconnecting")
                setStatus(Status.RECONNECTING);
                setDisconnected(false);
            }, 1000);

            return () => {
                clearTimeout(timeout);
            }
        }
        else {
            console.log("connecting to publisher");
            let socket = new WebSocket(props.url);

            socket.onopen = () => {
                console.log("connected");
                setStatus(Status.CONNECTED);
            }

            socket.onclose = () => {
                console.log("disconnected");
                setStatus(Status.DISCONNECTED);
                setDisconnected(true);
                onClear();
            }

            socket.onmessage = message => {
                const event = JSON.parse(message.data);
                if (event === "Clear") onClear();
                else if ("Spawn" in event) onSpawn(event["Spawn"]);
                else if ("Kill" in event) onKill(event["Kill"]);
                else if ("Frame" in event) onFrame(event["Frame"]);
                else if ("Settings" in event) onSettings(event["Settings"]);
                else console.log("unknown message: ", event);
            };

            socket.onerror = (error) => console.log("error: " + error);

            return () => {
                console.log("disconnecting");
                socket.close();
            }
        }
    }, [disconnected]);

    let issue = null;
    switch (status) {
        case Status.CONNECTING:
            issue = <div className="Issue">Connecting...</div>
            break;
        case Status.RECONNECTING:
            issue = <div className="Issue">Reconnecting...</div>
            break;
        case Status.DISCONNECTED:
            issue = <div className="Issue">Connection lost</div>
            break;
        default:
    }

    return (
        <div className="Viewer">
            <div className="Panes">
                <UI highlight={highlight} onHighlight={id => setHighlight(id)} agents={agents}/>
                <World settings={settings} agents={agents} frame={frame} highlight={highlight} onHighlight={id => setHighlight(id)}/>
            </div>
            {issue}
        </div>
    );
}
