import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <div>
        <h2>Upload file with websocket to localhost</h2>
        <div>
            <input type="file" id="file" name="file" />
            <button onClick={sendFile}>Send</button>
        </div>
      </div>
    </div>
  );
}

function sendFile() {
    const file = document.getElementById('file').files[0];
    const ws = new WebSocket('ws://localhost:8080/upload', 'binary');
    ws.binaryType = 'arraybuffer';
    ws.onopen = () => {
        console.log('connected');
        ws.send(file);
    };
    ws.onmessage = (event) => {
        console.log('received', event.data);
    };
    ws.onclose = () => {
        console.log('disconnected');
    };
    ws.onerror = (error) => {
        console.error('Error', error);
    };
}

export default App;
