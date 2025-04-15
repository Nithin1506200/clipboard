import React from "react";
import { Data } from "./bindings";
import { Editor, EditorProps } from "@monaco-editor/react";
import { CopyIcon, DeleteIcon } from "./assets/Icons";
type copyFn = (val: string) => void;
interface MyEditorProps extends Omit<EditorProps, "theme"> {
  onCopy?: () => void;
  onDelete?: () => void;
  onLanguageChange?: (val: string) => void;
}
const allLanguages = [
  "Rust",
  "C++",
  "C",
  "JavaScript",
  "TypeScript",
  "Python",
  "Go",
  "Java",
  "Kotlin",
  "Swift",
  "Ruby",
  "PHP",
  "C#",
  "HTML",
  "CSS",
  "SQL",
  "JSON",
  "Markdown",
];
export function MyEditor({
  onCopy,
  onDelete,
  onLanguageChange,
  ...editorProps
}: MyEditorProps) {
  return (
    <div className="relative group">
      <div className="absolute top-2 right-2 flex space-x-2 opacity-0 group-hover:opacity-100 transition-opacity z-[100]">
        {onLanguageChange && (
          <select
            onChange={(e) => onLanguageChange(e.target.value)}
            value={editorProps.language}
          >
            {allLanguages.map((e) => (
              <option key={e} value={e}>
                {e}
              </option>
            ))}
          </select>
        )}
        {onCopy && <CopyIcon strokeColor="black" onClick={(_) => onCopy()} />}
        {onDelete && (
          <DeleteIcon strokeColor="black" onClick={() => onDelete()} />
        )}
      </div>
      <Editor
        {...editorProps}
        theme="vs-dark"
        options={{ ...editorProps.options, minimap: { enabled: false } }}
      ></Editor>
    </div>
  );
}
function dataSegregate(data: Data) {
  switch (data.tag) {
    case "Email": {
      return <MyEditor value={data.content} />;
    }

    case "PhoneNumber": {
      {
        return <MyEditor value={data.content} />;
      }
    }
    case "JsonDict": {
      {
        return <MyEditor value={data.content as string} language="json" />;
      }
    }
    case "Code": {
      {
        return (
          <MyEditor
            onCopy={() => {}}
            onDelete={() => {}}
            onLanguageChange={(lang) => {}}
            height={"100px"}
            width={"100%"}
            value={data.content.data as string}
            language={"rust"}
            options={{
              readOnly: true,
            }}
          />
        );
      }
    }
  }
}
const UnMemoizedDataDisplay: React.FC<{ data: Data }> = ({
  data,
}: {
  data: Data;
}) => {
  //   let data;
  return (
    <div className="rounded border-2 border-black">{dataSegregate(data)}</div>
  );
};

let DataDisplay = React.memo(UnMemoizedDataDisplay);

export default DataDisplay;
