import React, { useState, useEffect } from "react";

import { AllData, commands, Data, Result } from ".././bindings";
import DataDisplay, { MyEditor } from "../components/MonacoDisplay";
import PoolClipboardBool from ".././components/PoolClipboard";

function Search() {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState<Result<Data[], string>>({
    status: "ok",
    data: [],
  });

  const handleSearch = async () => {
    if (query.trim() !== "") {
      let res = await commands.fuzzySearch(query);
      setResult((_) => res);
      console.log("log", res);
    }
  };
  let renderResult = () => {
    switch (result.status) {
      case "ok":
        return result.data.map((data) => <DataDisplay data={data} />);
      case "error":
    }
  };
  return (
    <div style={{ marginBottom: "20px" }}>
      <input
        type="text"
        placeholder="Enter search query"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        style={{ padding: "8px", marginRight: "8px", width: "200px" }}
      />
      <button onClick={handleSearch} style={{ padding: "8px 16px" }}>
        Search
      </button>
      <code>{renderResult()}</code>
    </div>
  );
}

function Clipboard() {
  const [allData, setAllData] = useState<AllData[]>([]);
  useEffect(() => {
    async function fetchIds() {
      try {
        const data = await commands.getAllData(0, 10);
        setAllData(data);
      } catch (error) {
        console.error("Error fetching IDs:", error);
      }
    }
    fetchIds();
  }, []);
  return (
    <main>
      <PoolClipboardBool />
      <h1>Clipboard Data</h1>

      {/* Display the list of IDs */}
      <div>
        <h2>All Data:</h2>
        <ul>
          {allData.map((data) => (
            <li key={data.id} className=" ">
              <div>{data.id}</div>
              <DataDisplay data={data.data} />
            </li>
          ))}
        </ul>
      </div>
      <Search />
    </main>
  );
}

export default Clipboard;
