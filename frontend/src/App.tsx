import { ChenRouter, Routes, Route } from 'chen-the-dawnstreak';
import './App.css';

function Home() {
  return (
    <div style={{ padding: '2rem', maxWidth: 600, margin: '2rem auto' }}>
      <h1>欢迎使用赤刃明霄陈</h1>
      <p>轻量级 React 元框架</p>
    </div>
  );
}

function App() {
  return (
    <ChenRouter>
      <Routes>
        <Route path="/" element={<Home />} />
      </Routes>
    </ChenRouter>
  );
}

export default App;
