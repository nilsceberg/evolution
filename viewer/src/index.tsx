import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import { Viewer } from './Viewer';
import reportWebVitals from './reportWebVitals';

const params = new URLSearchParams(window.location.search);
const url = params.get("url") || `ws://${window.location.hostname}:29999`;

ReactDOM.render(
    <React.StrictMode>
        <Viewer url={url} />
    </React.StrictMode>,
    document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
