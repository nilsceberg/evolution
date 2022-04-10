import { useEffect, useState } from 'react';
import './Viewer.css';
import { World } from "./World";

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

    const [agents, setAgents] = useState([]);
    const [frame, setFrame] = useState([]);

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
            }

            socket.onmessage = message => {
                console.log("message: " + message.data);
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
            issue = <div className="issue">Connecting...</div>
            break;
        case Status.RECONNECTING:
            issue = <div className="issue">Reconnecting...</div>
            break;
        case Status.DISCONNECTED:
            issue = <div className="issue">Connection lost</div>
            break;
        default:
    }

    return (
        <div className="viewer">
            <World agents={agents} frame={frame}/>
            {issue}
        </div>
    );
}
