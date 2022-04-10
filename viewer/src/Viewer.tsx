import { useEffect } from 'react';
import './Viewer.css';

interface ViewerProps {
    url: string,
}

export const Viewer = (props: ViewerProps) => {
    useEffect(() => {
        console.log("connecting to publisher");
        let socket = new WebSocket(props.url);

        socket.onopen = () => console.log("connected");
        socket.onclose = () => console.log("disconnected");
        socket.onmessage = (message) => console.log("message: " + message);
        socket.onerror = (error) => console.log("error: " + error);

        return () => {
            socket.close();
        }
    });

    return (
        <div className="viewer">
            <header className="app-header">
                Brains
            </header>
        </div>
    );
}
