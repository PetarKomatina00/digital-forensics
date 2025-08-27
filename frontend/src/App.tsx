import { Link, Route, Routes } from "react-router-dom";
import "./App.css";
import { Home } from "./components/Home";
import { Histogram } from "./components/Histogram";

function App() {
  return (
    <div className="container">
      <Routes>
        <Route path="/" element={<Home/>} />
        <Route path="/histogram" element={<Histogram />} />
      </Routes>
    </div>
  );
}

export default App;
