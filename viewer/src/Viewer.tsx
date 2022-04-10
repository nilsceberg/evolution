import { useEffect, useState } from 'react';
import './Viewer.css';

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

    useEffect(() => {
        if (status !== Status.CONNECTING && status !== Status.RECONNECTING) {
            return;
        }

        console.log("connecting to publisher");
        let socket = new WebSocket(props.url);

        socket.onopen = () => {
            console.log("connected");
            setStatus(Status.CONNECTED);
        }

        socket.onclose = () => {
            console.log("disconnected");
            setStatus(Status.DISCONNECTED);
        }

        socket.onmessage = (message) => console.log("message: " + message);
        socket.onerror = (error) => console.log("error: " + error);

        return () => {
            socket.close();
        }
    }, [status]);

    useEffect(() => {
        if (status === Status.DISCONNECTED) {
            const timeout = setTimeout(() => {
                setStatus(Status.RECONNECTING);
            }, 1000);

            return () => {
                clearTimeout(timeout);
            }
        }
    }, [status]);

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
            {issue}
        </div>
    );
}
