import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";
import { Popup } from "./pages/Popup.page";
import "./App.css";
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Popup />
    </BrowserRouter>
  </React.StrictMode>
);
