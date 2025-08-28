import { Link, Route, Routes } from "react-router-dom";
import "./App.css";
import { Home } from "./components/Home";
import { Histogram } from "./components/Histogram";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer } from 'react-toastify';
function App() {
  return (
    <div className="container">
      <Routes>
        <Route path="/" element={<Home/>} />
        <Route path="/histogram" element={<Histogram />} />
      </Routes>
      <ToastContainer/>
    </div>
  );
}

export default App;
