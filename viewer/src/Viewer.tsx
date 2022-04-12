import { useEffect, useState } from 'react';
import { AgentInfo, Frame, Settings } from './types';
import './Viewer.css';
import { World } from "./World";
import { UI } from "./UI";

interface ViewerProps {
    url: string,
    ui: boolean,
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
    const [[frameNumber, frame], setFrame] = useState<[number, Frame]>([0, []]);
    const [settings, setSettings] = useState<Settings>({
        title: "",
        world_radius: 0,
        zone: undefined,
        frame_interval: 0,
        generation_time: 0,
        time_step: 0,
    });

    const [highlight, setHighlight] = useState<string>("");
    const [showUi, setShowUi] = useState(false);
    const [startTime, setStartTime] = useState(new Date());

    const onClear = () => {
        setAgents([]);
        setFrame([0, []]);
        setStartTime(new Date());
    };

    const onSpawn = (newAgents: AgentInfo[]) => {
        // Oh no, this is broken, I believe!
        setFrame([frameNumber, [...frame, ...newAgents.map(agent => [0, 0] as [number, number])]]);
        setAgents([...agents, ...newAgents]);
    }

    const onKill = (indices: number[]) => {
        // Oh no, this is broken, I believe!
        setAgents(agents.filter((_, i) => i in indices));
        setFrame([frameNumber, frame.filter((_, i) => i in indices)]);
    }

    const onFrame = (num: number, frame: Frame) => {
        // Oh no, this is broken!
        setFrame([num, frame]);
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
            let frameNumber = 0;

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
                if (event === "Clear") {
                    onClear();
                    frameNumber = 0;
                }
                else if ("Spawn" in event) onSpawn(event["Spawn"]);
                else if ("Kill" in event) onKill(event["Kill"]);
                else if ("Frame" in event) {
                    onFrame(++frameNumber, event["Frame"]);
                }
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

    console.log(frameNumber)

    return (
        <div className="Viewer">
            <div className="Panes">
                {props.ui ? <UI highlight={showUi ? highlight : ""} onHighlight={id => setHighlight(id)} agents={agents} show={showUi} onToggle={show => setShowUi(show)}/> : null}
                <World simulationTime={frameNumber * settings.time_step} startTime={startTime} highlight={showUi ? highlight : ""} settings={settings} agents={agents} frame={frame} onHighlight={id => setHighlight(id)}/>
            </div>
            {issue}
        </div>
    );
}
