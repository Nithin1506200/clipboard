import Clipboard from "./pages/Clipboard.page";
import "./App.css";
import { Route, Routes } from "react-router";
import { Popup } from "./pages/Popup.page";

function App() {
  return (
    <Routes>
      <Route index element={<Clipboard />} />
      <Route path="popup" element={<Popup />} />
    </Routes>
  );
}

export default App;
